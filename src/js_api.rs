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

mod request_data;
mod response_result;

use actix_web::{Responder, web, HttpResponse};
use crate::config::ProjectConfig;
use actix_session::Session;
use crate::db_api::DbConnection;
use serde::{Serialize, Deserialize};
use crate::security::{get_user_session, check_username, check_password, verify_password};
use crate::security::AccessLevel::User;
use crate::db_api::db_result::DbApiErrorType;
use crate::db_api::db_result::SessionErrorType::DbError;
use crate::js_api::request_data::LoginInfo;
use crate::js_api::response_result::BackendError;
use crate::js_api::response_result::ErrorCode::{DatabaseError, Unauthorized, UserInputError, NoResult, Ignored, UnknownError, CookieError};
use std::borrow::Borrow;

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
        let backend_error = BackendError::new(DatabaseError, error_txt.as_str());
        let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());

        return HttpResponse::InternalServerError().body(response_body);
    };
}

macro_rules! handle_session_error {
    ($user_session:ident) => {
        let error = $user_session.err().unwrap();
        let error_code = if error.error_type == DbError {
            DatabaseError
        }
        else {
            Unauthorized
        };

        let backend_error = BackendError::new(error_code, error.error_msg.as_str());
        let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());

        if error_code == DatabaseError {
            return HttpResponse::InternalServerError().body(response_body);
        }
        else {
            return HttpResponse::Forbidden().body(response_body);
        }
    };
}

pub async fn get_upload_data(config: web::Data<ProjectConfig>, session: Session, url_data: web::Path<i32>) -> impl Responder {
    let target_upload_id = url_data.into_inner();

    // Input checks
    if target_upload_id < 0 {
        let backend_error = BackendError::new(UserInputError, "Die Upload-ID muss 0 oder größer sein");
        let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());
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
            let backend_error = BackendError::new(NoResult, error.error_msg.as_str());
            let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());

            return HttpResponse::NotFound().body(response_body);
        }
        else {
            let backend_error = BackendError::new(DatabaseError, error.error_msg.as_str());
            let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());

            return HttpResponse::InternalServerError().body(response_body);
        }
    }
}

pub async fn login(config: web::Data<ProjectConfig>, session: Session, login_data: web::Form<LoginInfo>) -> impl Responder {
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
                            let backend_error = BackendError::new(CookieError, "Fehler beim Setzen des Session Cookies");
                            let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());

                            return HttpResponse::InternalServerError().body(response_body);
                        }
                    }
                    else {
                        let backend_error = BackendError::new(DatabaseError, "Fehler beim Anlegen der Session in der Redis Datenbank");
                        let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());

                        return HttpResponse::InternalServerError().body(response_body);
                    }
                }
                else {
                    let backend_error = BackendError::new(UserInputError, "Benutzername oder Passwort ist falsch");
                    let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());

                    return HttpResponse::Forbidden().body(response_body);
                }
            }
            else {
                let error = user_data.err().unwrap();
                let error_type = error.error_type;

                if error_type == DbApiErrorType::NoResult {
                    let backend_error = BackendError::new(UserInputError, "Benutzername oder Passwort ist falsch");
                    let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());

                    return HttpResponse::Forbidden().body(response_body);
                }
                else {
                    let backend_error = BackendError::new(DatabaseError, error.error_msg.as_str());
                    let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());

                    return HttpResponse::InternalServerError().body(response_body);
                }
            }
        }
        else {
            let backend_error = BackendError::new(UserInputError, "Ungültige Zeichen in Benutzername oder Passwort");
            let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());

            return HttpResponse::Ok().body(response_body);
        }
    }
    else {
        let backend_error = BackendError::new(Ignored, "Es ist bereits ein Benutzer eingelogt");
        let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());

        return HttpResponse::Ok().body(response_body);
    }
}

pub async fn logout(config: web::Data<ProjectConfig>, session: Session) -> impl Responder {
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
        let backend_error = BackendError::new(DatabaseError, redis_error.error_msg.as_str());
        let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());

        return HttpResponse::InternalServerError().body(response_body);
    }
}