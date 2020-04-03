/*
 * Copyright (C) 2020 Voldracarno Draconor <ThaFireDragonOfDeath@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

pub mod request_data;
pub mod response_result;

use actix_web::{web, HttpResponse, HttpMessage};
use crate::config::ProjectConfig;
use actix_session::Session;
use crate::db_api::DbConnection;
use crate::security::{get_user_session, check_username, check_password, verify_password, check_invite_key, hash_password, check_filename};
use crate::db_api::db_result::DbApiErrorType;
use crate::db_api::db_result::SessionErrorType::DbError;
use crate::js_api::request_data::{LoginData, RegisterData, check_file_mime, check_form_content_mime, TagData};
use crate::js_api::response_result::{BackendError, AddUploadSuccess, UserData};
use crate::js_api::response_result::ErrorCode::{DatabaseError, Unauthorized, UserInputError, NoResult, Ignored, UnknownError, CookieError, InternalError};
use actix_multipart::{Multipart, Field};
use futures::{StreamExt, TryStreamExt};
use std::collections::HashMap;
use log::{trace, debug, info, warn, error};
use crate::tokio::io::AsyncWriteExt;
use crate::file_api::{get_upload_path_tmp, process_file, delete_upload_srv};
use crate::file_api::FileProcessErrorType::FormatError;
use crate::db_api::db_result::DbApiErrorType::PartFail;

macro_rules! get_db_connection {
    ($config:ident, $req_postgres:expr, $req_redis:expr) => {
        {
            let db_connection = DbConnection::new($config.as_ref(), $req_postgres, $req_redis).await;

            if db_connection.is_err() {
                handle_db_connection_error!(db_connection);
            }

            db_connection.ok().unwrap()
        }
    };
}

macro_rules! get_user_session_data {
    ($db_connection:ident, $session:ident, $force_session_renew:expr) => {
        {
            let user_session = get_user_session(&$db_connection, &$session, $force_session_renew).await;

            if user_session.is_err() {
                handle_session_error!(user_session);
            }

            user_session.ok().unwrap()
        }
    };
}

macro_rules! handle_db_connection_error {
    ($db_connection:ident) => {
        let error = $db_connection.err().unwrap();
        let error_txt = error.error_msg;

        handle_error_str!(DatabaseError, error_txt.as_str(), InternalServerError);
    };
}

macro_rules! handle_error_str {
    ($error_code:expr, $error_str:expr, $http_code:ident) => {
        let backend_error = BackendError::new($error_code, $error_str);
        let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());

        return HttpResponse::$http_code().body(response_body);
    };
}

macro_rules! handle_session_error {
    ($user_session:ident) => {
        let error = $user_session.err().unwrap();
        let error_txt = error.error_msg;
        let error_code = if error.error_type == DbError {
            DatabaseError
        }
        else {
            Unauthorized
        };

        if error_code == DatabaseError {
            handle_error_str!(error_code, error_txt.as_str(), InternalServerError);
        }
        else {
            handle_error_str!(error_code, error_txt.as_str(), Forbidden);
        }
    };
}

pub async fn add_upload(config: web::Data<ProjectConfig>, session: Session, mut payload: Multipart) -> HttpResponse {
    let db_connection = get_db_connection!(config, true, true);
    let session_data = get_user_session_data!(db_connection, session, false);
    let multipart_data = parse_multipart_form_data(&mut payload).await;
    let taglist_str = multipart_data.get("taglist");
    let filename = multipart_data.get("file");
    let upload_classification = multipart_data.get("classification");
    let mut upload_is_nsfw = false;

    if upload_classification.is_some() {
        let upload_classification = upload_classification.unwrap();

        if upload_classification == "nsfw" {
            upload_is_nsfw = true;
        }
        else if  upload_classification != "sfw" {
            handle_error_str!(UserInputError, "Upload muss entweder als SFW oder als NSFW gekennzeichnet sein", BadRequest);
        }
    }
    else {
        handle_error_str!(UserInputError, "Upload muss entweder als SFW oder als NSFW gekennzeichnet sein", BadRequest);
    }

    if filename.is_some() {
        let filename = filename.unwrap();
        let filename_is_ok = check_filename(filename.as_str());

        if filename_is_ok {
            let file_process_success = process_file(&config, filename).await;

            if file_process_success.is_ok() {
                let uploader_id = session_data.user_id;
                let db_success = db_connection.add_upload(filename, upload_is_nsfw, uploader_id).await;

                if db_success.is_ok() {
                    let upload_id = db_success.ok().unwrap();

                    if taglist_str.is_some() {
                        let taglist_str = taglist_str.unwrap();
                        let taglist_data = TagData::from_str(taglist_str);
                        let taglist_full_success = taglist_data.full_success;
                        let taglist_vec = taglist_data.as_str_ref_vec();

                        let db_success = db_connection.add_tags(taglist_vec, uploader_id, upload_id).await;

                        if db_success.is_ok() {
                            let ret_val = AddUploadSuccess::new(true, true, taglist_full_success);
                            let response_txt = serde_json::to_string(&ret_val).unwrap_or("".to_owned());

                            return HttpResponse::Ok().body(response_txt);
                        }
                        else {
                            let error = db_success.err().unwrap();
                            let error_type = error.error_type;

                            if error_type == PartFail {
                                let ret_val = AddUploadSuccess::new(true, true, false);
                                let response_txt = serde_json::to_string(&ret_val).unwrap_or("".to_owned());

                                return HttpResponse::Ok().body(response_txt);
                            }
                        }
                    }

                    let ret_val = AddUploadSuccess::new(true, false, false);
                    let response_txt = serde_json::to_string(&ret_val).unwrap_or("".to_owned());

                    return HttpResponse::Ok().body(response_txt);
                }
                else {
                    delete_upload_srv(&config, filename).await;

                    let error = db_success.err().unwrap();
                    let error_msg = error.error_msg;
                    handle_error_str!(InternalError, error_msg.as_str(), InternalServerError);
                }
            }
            else {
                let error = file_process_success.unwrap_err();
                let error_type = error.error_code;
                let error_msg = error.error_msg;

                if error_type == FormatError {
                    handle_error_str!(UserInputError, error_msg.as_str(), BadRequest);
                }
                else {
                    handle_error_str!(InternalError, error_msg.as_str(), InternalServerError);
                }
            }
        }
        else {
            handle_error_str!(UserInputError, "Dateiname enthält ungültige Zeichen", BadRequest);
        }
    }
    else {
        handle_error_str!(UnknownError, "Es ist ein Fehler beim Speichern der Datei auf dem Server aufgetreten", InternalServerError);
    }
}

pub async fn get_uploads(config: web::Data<ProjectConfig>, session: Session, url_data: web::Path<(i32, i16, bool)>) -> HttpResponse {
    let db_connection = get_db_connection!(config, true, true);
    let session_data = get_user_session_data!(db_connection, session, false);

    let (start_id, amount, show_nsfw) = url_data.into_inner();

    if start_id < 1 {
        handle_error_str!(UserInputError, "Die Start ID kann nicht kleiner als 1 sein", BadRequest);
    }

    if amount < 1 || amount > 500 {
        handle_error_str!(UserInputError, "Die Anzahl der auszugebenden Uploads muss im Bereich von 1 bis 500 liegen", BadRequest);
    }

    let uploads = db_connection.get_uploads(start_id, amount, show_nsfw).await;

    if uploads.is_ok() {
        let uploads = uploads.ok().unwrap();
        let response_txt = serde_json::to_string(&uploads).unwrap_or("".to_owned());

        return HttpResponse::Ok().body(response_txt);
    }
    else {
        let error = uploads.err().unwrap();

        if error.error_type == DbApiErrorType::NoResult {
            handle_error_str!(NoResult, error.error_msg.as_str(), NotFound);
        }
        else {
            handle_error_str!(DatabaseError, error.error_msg.as_str(), InternalServerError);
        }
    }
}

pub async fn get_upload_data(config: web::Data<ProjectConfig>, session: Session, url_data: web::Path<i32>) -> HttpResponse {
    let target_upload_id = url_data.into_inner();

    // Input checks
    if target_upload_id < 1 {
        handle_error_str!(UserInputError, "Die Upload-ID muss 1 oder größer sein", BadRequest);
    }

    let db_connection = get_db_connection!(config, true, true);
    let session_data = get_user_session_data!(db_connection, session, false);
    let upload_data = db_connection.get_upload_data(target_upload_id).await;

    if upload_data.is_ok() {
        let upload_data = upload_data.ok().unwrap();
        let response_txt = serde_json::to_string(&upload_data).unwrap_or("".to_owned());

        return HttpResponse::Ok().body(response_txt);
    }
    else {
        let error = upload_data.err().unwrap();

        if error.error_type == DbApiErrorType::NoResult {
            handle_error_str!(NoResult, error.error_msg.as_str(), NotFound);
        }
        else {
            handle_error_str!(DatabaseError, error.error_msg.as_str(), InternalServerError);
        }
    }
}

pub async fn get_userdata_by_username(config: web::Data<ProjectConfig>, session: Session, url_data: web::Path<String>) -> HttpResponse {
    let target_username = url_data.into_inner();
    let username_is_ok = check_username(target_username.as_str());

    if username_is_ok {
        let db_connection = get_db_connection!(config, true, true);
        let session_data = get_user_session_data!(db_connection, session, false);
        let user_data = db_connection.get_userdata_by_username(target_username.as_str()).await;

        if user_data.is_ok() {
            let user_data = UserData::new(&user_data.ok().unwrap());
            let response_txt = serde_json::to_string(&user_data).unwrap_or("".to_owned());

            return HttpResponse::Ok().body(response_txt);
        }
        else {
            let error = user_data.err().unwrap();
            let error_type = error.error_type;
            let error_msg = error.error_msg;

            if error_type == DbApiErrorType::NoResult {
                handle_error_str!(NoResult, error_msg.as_str(), NotFound);
            }
            else {
                handle_error_str!(DatabaseError, error_msg.as_str(), InternalServerError);
            }
        }
    }
    else {
        handle_error_str!(UserInputError, "Benutzername ist ungültig", BadRequest);
    }
}

pub async fn login(config: web::Data<ProjectConfig>, session: Session, login_data: web::Form<LoginData>) -> HttpResponse {
    let db_connection = get_db_connection!(config, true, true);
    let user_session = get_user_session(&db_connection, &session, false).await;

    if user_session.is_err() {
        let login_username = login_data.username.clone();
        let login_password = login_data.password.clone();
        let keep_logged_in = login_data.keep_logged_in;

        let username_is_ok = check_username(login_username.as_str());
        let password_is_ok = check_password(login_password.as_str());

        if username_is_ok && password_is_ok {
            let user_data = db_connection.get_userdata_by_username(login_username.as_str()).await;

            if user_data.is_ok() {
                let user_data = user_data.ok().unwrap();
                let password_hash = user_data.password_hash.clone();
                let secret_hash_key = config.security_config.password_hash_key.get_value();
                let password_is_correct = verify_password(password_hash.as_str(),
                                                          login_password.as_str(),
                                                          secret_hash_key.as_str()).unwrap_or(false);

                if password_is_correct {
                    let user_id = user_data.user_id;
                    let session_data = db_connection.create_session(user_id, keep_logged_in).await;

                    if session_data.is_ok() {
                        let session_data = session_data.ok().unwrap();
                        let session_id = session_data.session_id.clone();
                        let session_set_result = session.set("session_id", session_id.as_str());

                        if session_set_result.is_ok() {
                            let response_userdata = response_result::UserData::new(&user_data);
                            let response_body = serde_json::to_string(&response_userdata).unwrap_or("".to_owned());

                            return HttpResponse::Ok().body(response_body);
                        }
                        else {
                            handle_error_str!(CookieError, "Fehler beim Setzen des Session Cookies", InternalServerError);
                        }
                    }
                    else {
                        handle_error_str!(DatabaseError, "Fehler beim Anlegen der Session in der Redis Datenbank", InternalServerError);
                    }
                }
                else {
                    handle_error_str!(UserInputError, "Benutzername oder Passwort ist falsch", Forbidden);
                }
            }
            else {
                let error = user_data.err().unwrap();
                let error_type = error.error_type;

                if error_type == DbApiErrorType::NoResult {
                    handle_error_str!(UserInputError, "Benutzername oder Passwort ist falsch", Forbidden);
                }
                else {
                    handle_error_str!(DatabaseError, error.error_msg.as_str(), InternalServerError);
                }
            }
        }
        else {
            handle_error_str!(UserInputError, "Ungültige Zeichen in Benutzername oder Passwort", BadRequest);
        }
    }
    else {
        handle_error_str!(Ignored, "Es ist bereits ein Benutzer eingelogt", BadRequest);
    }
}

pub async fn logout(config: web::Data<ProjectConfig>, session: Session) -> HttpResponse {
    let db_connection = get_db_connection!(config, false, true);
    let session_data = get_user_session_data!(db_connection, session, false);
    let session_id = session_data.session_id;
    let logoff_result = db_connection.destroy_session(session_id.as_str()).await;

    if logoff_result.is_ok() {
        session.remove("session_id");

        return HttpResponse::Ok().body("{ \"success:\" true }");
    }
    else {
        let redis_error = logoff_result.err().unwrap();

        handle_error_str!(DatabaseError, redis_error.error_msg.as_str(), InternalServerError);
    }
}

//noinspection ALL
// Returns a map of name and content (in case of file: content = filename)
async fn parse_multipart_form_data(payload: &mut Multipart) -> HashMap<String, String> {
    let mut result_map : HashMap<String, String> = HashMap::new();

    while let Ok(Some(field)) = payload.try_next().await {
        let mut field : Field = field; // Hack to show types in IDEA IDE
        let content_disposition = field.content_disposition();

        if content_disposition.is_some() {
            let content_disposition = content_disposition.unwrap();

            if content_disposition.is_form_data() {
                let name = content_disposition.get_name();
                let filename = content_disposition.get_filename();
                let mime_type = field.content_type();
                let mut parse_full_success = true;
                let mut data_content = String::new();

                if name.is_some() {
                    if filename.is_some() {
                        let mime_type_is_ok = check_file_mime(mime_type);

                        if mime_type_is_ok {
                            // Warning: IntelliJ cant show types or perform code completion on async fs stuff,
                            // because tokio uses cfg attributes which the IDE can't parse (yet)

                            let filename = filename.unwrap().to_owned();
                            let filepath = get_upload_path_tmp(filename.as_str());
                            let file = tokio::fs::File::create(filepath.as_str()).await;

                            if file.is_ok() {
                                let mut file : tokio::fs::File = file.unwrap();

                                // Field in turn is stream of bytes
                                while let Some(chunk) = field.next().await {
                                    let data = chunk.unwrap();
                                    let write_result = file.write_all(&data).await;

                                    if write_result.is_err() {
                                        parse_full_success = false;
                                        let remove_result : Result<_, _> = tokio::fs::remove_file(filepath.as_str()).await;

                                        if remove_result.is_err() {
                                            error!("Can't delete file: {}", filepath.as_str());
                                        }

                                        break;
                                    }
                                }

                                if parse_full_success {
                                    result_map.insert(name.unwrap().to_owned(), filename.clone());
                                }
                            }
                        }
                    }
                    else {
                        let mime_type_is_ok = check_form_content_mime(mime_type);

                        if mime_type_is_ok {
                            while let Some(chunk) = field.next().await {
                                let data = chunk.unwrap();
                                let parse_result = String::from_utf8(data.to_vec());

                                if parse_result.is_ok() {
                                    let read_data = parse_result.unwrap();
                                    data_content.push_str(read_data.as_str());
                                } else {
                                    parse_full_success = false;
                                    break;
                                }
                            }

                            if parse_full_success {
                                result_map.insert(name.unwrap().to_owned(), data_content);
                            }
                        }
                    }
                }
            }
        }
    }

    return result_map;
}

pub async fn register(config: web::Data<ProjectConfig>, register_data: web::Form<RegisterData>) -> HttpResponse {
    let db_connection = get_db_connection!(config, true, false);

    let username = register_data.username.clone();
    let password = register_data.password.clone();
    let invite_key = register_data.invite_key.clone();

    let username_is_ok = check_username(username.as_str());
    let password_is_ok = check_password(password.as_str());
    let invite_key_is_ok = check_invite_key(invite_key.as_str());

    if username_is_ok && password_is_ok && invite_key_is_ok {
        let master_invite_key = config.security_config.master_invite_key.get_value();

        if invite_key == master_invite_key {
            let password_hash_key = config.security_config.password_hash_key.get_value();
            let password_hash = hash_password(password.as_str(), password_hash_key.as_str()).unwrap_or(String::new());

            if password_hash != "" {
                let create_result = db_connection.add_user(username.as_str(),password_hash.as_str(), false).await;

                if create_result.is_ok() {
                    return HttpResponse::Ok().body("{ \"success:\" true }");
                }
                else {
                    handle_error_str!(DatabaseError, "Fehler beim Anlegen des Benutzers in der Datenbank", InternalServerError);
                }
            }
            else {
                handle_error_str!(DatabaseError, "Fehler beim Hashen des Passwortes", InternalServerError);
            }
        }
        else {
            handle_error_str!(UserInputError, "Der eingegebene Invitecode ist ungültig", Forbidden);
        }
    }
    else {
        handle_error_str!(UserInputError, "Die eingegebenen Daten entsprechen nicht den Richtlinien", BadRequest);
    }
}