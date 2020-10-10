pub mod request_data;
pub mod response_result;

use actix_web::{web, HttpResponse};
use crate::config::ProjectConfig;
use actix_session::Session;
use crate::db_api::DbConnection;
use crate::security::{get_user_session, check_username, check_password, verify_password, check_invite_key, hash_password, check_filename};
use crate::db_api::db_result::{DbApiErrorType, UploadPrvList, UploadData};
use crate::db_api::db_result::SessionErrorType::DbError;
use crate::backend_api::response_result::ErrorCode::{DatabaseError, Unauthorized, UserInputError, NoResult, Ignored, UnknownError, CookieError, InternalError};
use actix_multipart::{Multipart, Field};
use futures::{StreamExt, TryStreamExt};
use std::collections::HashMap;
use log::{error};
use tokio::io::AsyncWriteExt;
use crate::file_api::{get_upload_path_tmp, process_file, delete_upload_srv};
use crate::file_api::FileProcessErrorType::FormatError;
use crate::db_api::db_result::DbApiErrorType::PartFail;
use std::ops::Deref;
use crate::backend_api::request_data::{CommentData, TagData, LoginData, RegisterData, check_file_mime, check_form_content_mime,};
use crate::backend_api::response_result::{BackendError, SuccessReport, AddUploadSuccess, UserExists, Filter, UserData};
use actix_web::http::StatusCode;

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

        handle_error_str!(DatabaseError, error_txt.as_str(), INTERNAL_SERVER_ERROR);
    };
}

macro_rules! handle_error_str {
    ($error_code:expr, $error_str:expr, $http_code:ident) => {
        let backend_error = BackendError::new(StatusCode::$http_code.as_u16(), $error_code, $error_str);

        return Err(backend_error);
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
            handle_error_str!(error_code, error_txt.as_str(), INTERNAL_SERVER_ERROR);
        }
        else {
            handle_error_str!(error_code, error_txt.as_str(), FORBIDDEN);
        }
    };
}

pub async fn add_comment(config: &web::Data<ProjectConfig>, session: &Session, comment_data: &web::Form<CommentData>) -> Result<SuccessReport, BackendError> {
    let db_connection = get_db_connection!(config, true, true);
    let session_data = get_user_session_data!(db_connection, session, false);

    let validated_comment_data = comment_data.validate_data(&db_connection).await;

    if validated_comment_data.is_some() {
        let validated_comment_data = validated_comment_data.unwrap();
        let comment_upload = validated_comment_data.upload_id;
        let comment_text = validated_comment_data.comment_text;
        let comment_poster = session_data.user_id;

        let post_result = db_connection.add_comment(comment_poster, comment_upload, comment_text.as_str()).await;

        if post_result.is_ok() {
            return Ok(SuccessReport::new(true));
        }
        else {
            let error = post_result.err().unwrap();
            let error_msg = error.error_msg;

            handle_error_str!(InternalError, error_msg.as_str(), INTERNAL_SERVER_ERROR);
        }
    }
    else {
        handle_error_str!(UserInputError, "Übergebene Daten konnten nicht validiert werden", BAD_REQUEST);
    }
}

pub async fn add_upload(config: &web::Data<ProjectConfig>, session: &Session, payload: &mut Multipart) -> Result<AddUploadSuccess, BackendError> {
    let db_connection = get_db_connection!(config, true, true);
    let session_data = get_user_session_data!(db_connection, session, false);
    let multipart_data = parse_multipart_form_data(payload, false).await;
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
            handle_error_str!(UserInputError, "Upload muss entweder als SFW oder als NSFW gekennzeichnet sein", BAD_REQUEST);
        }
    }
    else {
        handle_error_str!(UserInputError, "Upload muss entweder als SFW oder als NSFW gekennzeichnet sein", BAD_REQUEST);
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

                            return Ok(ret_val);
                        }
                        else {
                            let error = db_success.err().unwrap();
                            let error_type = error.error_type;

                            if error_type == PartFail {
                                let ret_val = AddUploadSuccess::new(true, true, false);

                                return Ok(ret_val);
                            }
                        }
                    }

                    let ret_val = AddUploadSuccess::new(true, false, false);

                    return Ok(ret_val);
                }
                else {
                    delete_upload_srv(&config, filename).await;

                    let error = db_success.err().unwrap();
                    let error_msg = error.error_msg;
                    handle_error_str!(InternalError, error_msg.as_str(), INTERNAL_SERVER_ERROR);
                }
            }
            else {
                let error = file_process_success.unwrap_err();
                let error_type = error.error_code;
                let error_msg = error.error_msg;

                if error_type == FormatError {
                    handle_error_str!(UserInputError, error_msg.as_str(), BAD_REQUEST);
                }
                else {
                    handle_error_str!(InternalError, error_msg.as_str(), INTERNAL_SERVER_ERROR);
                }
            }
        }
        else {
            handle_error_str!(UserInputError, "Dateiname enthält ungültige Zeichen", BAD_REQUEST);
        }
    }
    else {
        handle_error_str!(UnknownError, "Es ist ein Fehler beim Speichern der Datei auf dem Server aufgetreten", INTERNAL_SERVER_ERROR);
    }
}

pub async fn check_username_exists(config: &web::Data<ProjectConfig>, url_data: &web::Path<String>) -> Result<UserExists, BackendError> {
    let db_connection = get_db_connection!(config, true, false);

    let username = url_data.as_str();
    let username_is_ok = check_username(username);

    if !username_is_ok {
        handle_error_str!(UserInputError, "Benutzername entspricht nicht den Richtlinien", BAD_REQUEST);
    }

    let user_exists = db_connection.check_user_exists(username).await;

    if user_exists.is_ok() {
        let user_exists = user_exists.ok().unwrap();
        let user_exists_obj = UserExists::new(user_exists);

        return Ok(user_exists_obj);
    }
    else {
        let error = user_exists.err().unwrap();
        let error_msg = error.error_msg;

        handle_error_str!(InternalError, error_msg.as_str(), INTERNAL_SERVER_ERROR);
    }
}

pub async fn get_filter(config: &web::Data<ProjectConfig>, session: &Session) -> Result<Filter, BackendError> {
    let db_connection = get_db_connection!(config, true, true);
    let _session_data = get_user_session_data!(db_connection, session, false);

    let show_sfw = session.get::<bool>("show_sfw");
    let show_nsfw = session.get::<bool>("show_nsfw");

    if show_sfw.is_ok() && show_nsfw.is_ok() {
        let show_sfw = show_sfw.unwrap();
        let show_nsfw = show_nsfw.unwrap();

        if show_sfw.is_some() && show_nsfw.is_some() {
            let show_sfw = show_sfw.unwrap();
            let show_nsfw = show_nsfw.unwrap();

            let filter_obj = Filter::new(show_sfw, show_nsfw);

            return Ok(filter_obj);
        }
    }

    // If filter is not set: Set show_sfw = true and show_nsfw to false
    let set_result_1 = session.set("show_sfw", true);
    let set_result_2 = session.set("show_nsfw", false);

    if set_result_1.is_err() || set_result_2.is_err() {
        handle_error_str!(CookieError, "Fehler beim Speichern der Default Einstellung", INTERNAL_SERVER_ERROR);
    }
    else {
        let filter_obj = Filter::new(true, false);

        return Ok(filter_obj);
    }
}

pub async fn get_uploads(config: &web::Data<ProjectConfig>, session: &Session, url_data: &web::Path<(i32, i16, bool, bool)>) -> Result<UploadPrvList, BackendError> {
    let db_connection = get_db_connection!(config, true, true);
    let _session_data = get_user_session_data!(db_connection, session, false);

    let (start_id, amount, show_sfw, show_nsfw) = url_data.as_ref().clone();

    if start_id < 1 {
        handle_error_str!(UserInputError, "Die Start ID kann nicht kleiner als 1 sein", BAD_REQUEST);
    }

    if amount < 1 || amount > 500 {
        handle_error_str!(UserInputError, "Die Anzahl der auszugebenden Uploads muss im Bereich von 1 bis 500 liegen", BAD_REQUEST);
    }

    let uploads = db_connection.get_uploads(start_id, amount, show_sfw, show_nsfw).await;

    if uploads.is_ok() {
        let uploads = uploads.ok().unwrap();

        return Ok(uploads);
    }
    else {
        let error = uploads.err().unwrap();

        if error.error_type == DbApiErrorType::NoResult {
            handle_error_str!(NoResult, error.error_msg.as_str(), NOT_FOUND);
        }
        else {
            handle_error_str!(DatabaseError, error.error_msg.as_str(), INTERNAL_SERVER_ERROR);
        }
    }
}

pub async fn get_uploads_range(config: &web::Data<ProjectConfig>, session: &Session, url_data: &web::Path<(i32, i32, bool, bool)>) -> Result<UploadPrvList, BackendError> {
    let db_connection = get_db_connection!(config, true, true);
    let _session_data = get_user_session_data!(db_connection, session, false);

    let (start_id, end_id, show_sfw, show_nsfw) = url_data.as_ref().clone();

    if start_id < 1 {
        handle_error_str!(UserInputError, "Die Start ID kann nicht kleiner als 1 sein", BAD_REQUEST);
    }

    if end_id < 1 {
        handle_error_str!(UserInputError, "Die End ID kann nicht kleiner als 1 sein", BAD_REQUEST);
    }

    let uploads = db_connection.get_uploads_range(start_id, end_id, show_sfw, show_nsfw).await;

    if uploads.is_ok() {
        let uploads = uploads.ok().unwrap();

        return Ok(uploads);
    }
    else {
        let error = uploads.err().unwrap();

        if error.error_type == DbApiErrorType::NoResult {
            handle_error_str!(NoResult, error.error_msg.as_str(), NOT_FOUND);
        }
        else {
            handle_error_str!(DatabaseError, error.error_msg.as_str(), INTERNAL_SERVER_ERROR);
        }
    }
}

pub async fn get_upload_data(config: &web::Data<ProjectConfig>, session: &Session, url_data: &web::Path<i32>) -> Result<UploadData, BackendError> {
    let target_upload_id = url_data.as_ref().clone();

    // Input checks
    if target_upload_id < 1 {
        handle_error_str!(UserInputError, "Die Upload-ID muss 1 oder größer sein", BAD_REQUEST);
    }

    let db_connection = get_db_connection!(config, true, true);
    let _session_data = get_user_session_data!(db_connection, session, false);
    let upload_data = db_connection.get_upload_data(target_upload_id).await;

    if upload_data.is_ok() {
        let upload_data = upload_data.ok().unwrap();

        return Ok(upload_data);
    }
    else {
        let error = upload_data.err().unwrap();

        if error.error_type == DbApiErrorType::NoResult {
            handle_error_str!(NoResult, error.error_msg.as_str(), NOT_FOUND);
        }
        else {
            handle_error_str!(DatabaseError, error.error_msg.as_str(), INTERNAL_SERVER_ERROR);
        }
    }
}

pub async fn get_userdata_by_username(config: &web::Data<ProjectConfig>, session: &Session, url_data: &web::Path<String>) -> Result<UserData, BackendError> {
    let target_username = url_data.as_str();
    let username_is_ok = check_username(target_username);

    if username_is_ok {
        let db_connection = get_db_connection!(config, true, true);
        let _session_data = get_user_session_data!(db_connection, session, false);
        let user_data = db_connection.get_userdata_by_username(target_username).await;

        if user_data.is_ok() {
            let user_data = UserData::new(&user_data.ok().unwrap());

            return Ok(user_data);
        }
        else {
            let error = user_data.err().unwrap();
            let error_type = error.error_type;
            let error_msg = error.error_msg;

            if error_type == DbApiErrorType::NoResult {
                handle_error_str!(NoResult, error_msg.as_str(), NOT_FOUND);
            }
            else {
                handle_error_str!(DatabaseError, error_msg.as_str(), INTERNAL_SERVER_ERROR);
            }
        }
    }
    else {
        handle_error_str!(UserInputError, "Benutzername ist ungültig", BAD_REQUEST);
    }
}

pub async fn login(config: &web::Data<ProjectConfig>, session: &Session, login_data: &web::Form<LoginData>) -> Result<UserData, BackendError> {
    let db_connection = get_db_connection!(config, true, true);
    let user_session = get_user_session(&db_connection, &session, false).await;

    if user_session.is_err() {
        let login_username = login_data.username.as_str();
        let login_password = login_data.password.as_str();
        let keep_logged_in = login_data.keep_logged_in;

        let username_is_ok = check_username(login_username);
        let password_is_ok = check_password(login_password);

        if username_is_ok && password_is_ok {
            let user_data = db_connection.get_userdata_by_username(login_username).await;

            if user_data.is_ok() {
                let user_data = user_data.ok().unwrap();
                let password_hash = user_data.password_hash.clone();
                let secret_hash_key = config.security_config.password_hash_key.get_value();
                let password_is_correct = verify_password(password_hash.as_str(),
                                                          login_password,
                                                          secret_hash_key.as_str()).unwrap_or(false);

                if password_is_correct {
                    let user_id = user_data.user_id;
                    let session_data = db_connection.create_session(user_id, keep_logged_in).await;

                    if session_data.is_ok() {
                        let session_data = session_data.ok().unwrap();
                        let session_id = session_data.session_id.clone();
                        let session_set_result_1 = session.set("session_id", session_id.as_str());
                        let session_set_result_2 = session.set("show_sfw", true);
                        let session_set_result_3 = session.set("show_nsfw", false);

                        if session_set_result_1.is_ok() && session_set_result_2.is_ok() && session_set_result_3.is_ok() {
                            let response_userdata = response_result::UserData::new(&user_data);

                            return Ok(response_userdata);
                        }
                        else {
                            session.purge();

                            handle_error_str!(CookieError, "Fehler beim Setzen des Session Cookies", INTERNAL_SERVER_ERROR);
                        }
                    }
                    else {
                        handle_error_str!(DatabaseError, "Fehler beim Anlegen der Session in der Redis Datenbank", INTERNAL_SERVER_ERROR);
                    }
                }
                else {
                    handle_error_str!(UserInputError, "Benutzername oder Passwort ist falsch", FORBIDDEN);
                }
            }
            else {
                let error = user_data.err().unwrap();
                let error_type = error.error_type;

                if error_type == DbApiErrorType::NoResult {
                    handle_error_str!(UserInputError, "Benutzername oder Passwort ist falsch", FORBIDDEN);
                }
                else {
                    handle_error_str!(DatabaseError, error.error_msg.as_str(), INTERNAL_SERVER_ERROR);
                }
            }
        }
        else {
            handle_error_str!(UserInputError, "Ungültige Zeichen in Benutzername oder Passwort", BAD_REQUEST);
        }
    }
    else {
        handle_error_str!(Ignored, "Es ist bereits ein Benutzer eingelogt", BAD_REQUEST);
    }
}

pub async fn logout(config: &web::Data<ProjectConfig>, session: &Session) -> Result<SuccessReport, BackendError> {
    let db_connection = get_db_connection!(config, false, true);
    let session_data = get_user_session_data!(db_connection, session, false);
    let session_id = session_data.session_id;
    let logoff_result = db_connection.destroy_session(session_id.as_str()).await;

    if logoff_result.is_ok() {
        session.purge();

        return Ok(SuccessReport::new(true));
    }
    else {
        let redis_error = logoff_result.err().unwrap();

        handle_error_str!(DatabaseError, redis_error.error_msg.as_str(), INTERNAL_SERVER_ERROR);
    }
}

//noinspection ALL
// Returns a map of name and content (in case of file: content = filename)
async fn parse_multipart_form_data(payload: &mut Multipart, allow_multiple_file_uploads: bool) -> HashMap<String, String> {
    let mut result_map : HashMap<String, String> = HashMap::new();
    let mut file_saved = false;

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
                    if filename.is_some() && (!file_saved || allow_multiple_file_uploads) {
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
                                    file_saved = true;
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

pub async fn register(config: &web::Data<ProjectConfig>, register_data: &web::Form<RegisterData>) -> Result<SuccessReport, BackendError> {
    let db_connection = get_db_connection!(config, true, false);

    let username = register_data.username.as_str();
    let password = register_data.password.as_str();
    let invite_key = register_data.invite_key.clone();

    let username_is_ok = check_username(username);
    let password_is_ok = check_password(password);
    let invite_key_is_ok = check_invite_key(invite_key.as_str());

    if username_is_ok && password_is_ok && invite_key_is_ok {
        let master_invite_key = config.security_config.master_invite_key.get_value();

        if invite_key == master_invite_key {
            let password_hash_key = config.security_config.password_hash_key.get_value();
            let password_hash = hash_password(password, password_hash_key.as_str()).unwrap_or(String::new());

            if password_hash != "" {
                let create_result = db_connection.add_user(username,password_hash.as_str(), false).await;

                if create_result.is_ok() {
                    return Ok(SuccessReport::new(true));
                }
                else {
                    handle_error_str!(DatabaseError, "Fehler beim Anlegen des Benutzers in der Datenbank", INTERNAL_SERVER_ERROR);
                }
            }
            else {
                handle_error_str!(DatabaseError, "Fehler beim Hashen des Passwortes", INTERNAL_SERVER_ERROR);
            }
        }
        else {
            handle_error_str!(UserInputError, "Der eingegebene Invitecode ist ungültig", FORBIDDEN);
        }
    }
    else {
        handle_error_str!(UserInputError, "Die eingegebenen Daten entsprechen nicht den Richtlinien", BAD_REQUEST);
    }
}

pub async fn set_filter(config: &web::Data<ProjectConfig>, session: &Session, url_data: &web::Path<(bool, bool)>) -> Result<SuccessReport, BackendError> {
    let (show_sfw, show_nsfw) = url_data.as_ref().clone();

    if show_sfw && show_nsfw {
        handle_error_str!(UserInputError, "Es muss mindestens ein Filter aktiviert sein", BAD_REQUEST);
    }

    let db_connection = get_db_connection!(config, true, true);
    let _session_data = get_user_session_data!(db_connection, session, false);

    let set_result_1 = session.set("show_sfw", show_sfw);
    let set_result_2 = session.set("show_nsfw", show_nsfw);

    if set_result_1.is_err() || set_result_2.is_err() {
        handle_error_str!(CookieError, "Fehler beim Speichern der Einstellung", INTERNAL_SERVER_ERROR);
    }

    return Ok(SuccessReport::new(true));
}

pub async fn vote_comment(config: &web::Data<ProjectConfig>, session: &Session, url_data: &web::Path<(i32, i32)>) -> Result<SuccessReport, BackendError> {
    let db_connection = get_db_connection!(config, true, true);
    let session_data = get_user_session_data!(db_connection, session, false);
    let (comment_id, vote_value) = url_data.as_ref().clone();
    let user_id = session_data.user_id;

    if comment_id < 1 {
        handle_error_str!(UserInputError, "Die Comment ID kann nicht kleiner als 1 sein", BAD_REQUEST);
    }

    if vote_value < 1 || vote_value > 1 {
        handle_error_str!(UserInputError, "Die Vote Nummer muss im Bereich von -1 und +1 liegen", BAD_REQUEST);
    }

    let db_result = db_connection.vote_comment(comment_id, user_id, vote_value).await;

    if db_result.is_ok() {
        return Ok(SuccessReport::new(true));
    }
    else {
        handle_error_str!(DatabaseError, "Fehler beim Speichern der Bewertung", INTERNAL_SERVER_ERROR);
    }
}

pub async fn vote_tag(config: &web::Data<ProjectConfig>, session: &Session, url_data: &web::Path<(i32, i32)>) -> Result<SuccessReport, BackendError> {
    let db_connection = get_db_connection!(config, true, true);
    let session_data = get_user_session_data!(db_connection, session, false);
    let (tum_id, vote_value) = url_data.as_ref().clone();
    let user_id = session_data.user_id;

    if tum_id < 1 {
        handle_error_str!(UserInputError, "Die Tag-Upload Map ID kann nicht kleiner als 1 sein", BAD_REQUEST);
    }

    if vote_value < 1 || vote_value > 1 {
        handle_error_str!(UserInputError, "Die Vote Nummer muss im Bereich von -1 und +1 liegen", BAD_REQUEST);
    }

    let db_result = db_connection.vote_tag(tum_id, user_id, vote_value).await;

    if db_result.is_ok() {
        return Ok(SuccessReport::new(true));
    }
    else {
        handle_error_str!(DatabaseError, "Fehler beim Speichern der Bewertung", INTERNAL_SERVER_ERROR);
    }
}

pub async fn vote_upload(config: &web::Data<ProjectConfig>, session: &Session, url_data: &web::Path<(i32, i32)>) -> Result<SuccessReport, BackendError> {
    let db_connection = get_db_connection!(config, true, true);
    let session_data = get_user_session_data!(db_connection, session, false);
    let (upload_id, vote_value) = url_data.as_ref().clone();
    let user_id = session_data.user_id;

    if upload_id < 1 {
        handle_error_str!(UserInputError, "Die Upload ID kann nicht kleiner als 1 sein", BAD_REQUEST);
    }

    if vote_value < 1 || vote_value > 1 {
        handle_error_str!(UserInputError, "Die Vote Nummer muss im Bereich von -1 und +1 liegen", BAD_REQUEST);
    }

    let db_result = db_connection.vote_upload(upload_id, user_id, vote_value).await;

    if db_result.is_ok() {
        return Ok(SuccessReport::new(true));
    }
    else {
        handle_error_str!(DatabaseError, "Fehler beim Speichern der Bewertung", INTERNAL_SERVER_ERROR);
    }
}