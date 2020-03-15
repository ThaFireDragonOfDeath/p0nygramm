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

use actix_web::{Responder, web, HttpResponse};
use crate::config::ProjectConfig;
use actix_session::Session;
use crate::db_api::DbConnection;
use serde::{Serialize, Deserialize};
use crate::js_api::ErrorCode::{DatabaseError, Unauthorized, UserInputError, NoResult};
use crate::security::get_user_session;
use crate::security::AccessLevel::User;
use crate::db_api::result::DbApiErrorType;
use crate::db_api::result::SessionErrorType::DbError;

#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum ErrorCode {
    DatabaseError,
    UserInputError,
    NoResult,
    Unauthorized,
}

#[derive(Serialize, Deserialize)]
pub struct BackendError {
    error_code: ErrorCode,
    error_msg: String,
}

impl BackendError {
    pub fn new(error_code: ErrorCode, error_msg: &str) -> BackendError {
        BackendError {
            error_code,
            error_msg: error_msg.to_owned(),
        }
    }
}

pub async fn get_upload_data(config: web::Data<ProjectConfig>, session: Session, url_data: web::Path<i32>) -> impl Responder {
    let db_connection = DbConnection::new(config.as_ref(), true, true).await;
    let target_upload_id = url_data.into_inner();

    // Input checks
    if target_upload_id < 0 {
        let backend_error = BackendError::new(UserInputError, "Die Upload-ID muss 0 oder größer sein");
        let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());
    }

    if db_connection.is_ok() {
        let db_connection = db_connection.ok().unwrap();
        let user_session = get_user_session(&db_connection, &session, false).await;

        if user_session.is_ok() {
            let session_data = user_session.ok().unwrap();
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
        else {
            let error = user_session.err().unwrap();
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
        }
    }
    else {
        let error = db_connection.err().unwrap();
        let error_txt = error.error_msg;
        let backend_error = BackendError::new(DatabaseError, error_txt.as_str());
        let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());

        return HttpResponse::InternalServerError().body(response_body);
    }
}

pub async fn logout(config: web::Data<ProjectConfig>, session: Session) -> impl Responder {
    let db_connection = DbConnection::new(config.as_ref(), false, true).await;

    if db_connection.is_ok() {
        let db_connection = db_connection.ok().unwrap();
        let user_session = get_user_session(&db_connection, &session, false).await;

        if user_session.is_ok() {
            let session_data = user_session.ok().unwrap();
            let session_id = session_data.session_id;
            let logoff_result = db_connection.destroy_session(session_id.as_str()).await;

            if logoff_result.is_ok() {
                return HttpResponse::Ok().body("{ \"success:\" true }");
            }
            else {
                let redis_error = logoff_result.err().unwrap();
                let backend_error = BackendError::new(DatabaseError, redis_error.error_msg.as_str());
                let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());

                return HttpResponse::InternalServerError().body(response_body);
            }
        }
        else {
            let error = user_session.err().unwrap();
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
        }
    }
    else {
        let error = db_connection.err().unwrap();
        let error_txt = error.error_msg;
        let backend_error = BackendError::new(DatabaseError, error_txt.as_str());
        let response_body = serde_json::to_string(&backend_error).unwrap_or("".to_owned());

        return HttpResponse::InternalServerError().body(response_body);
    }
}