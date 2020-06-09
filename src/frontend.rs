pub mod template_data;

use actix_web::{HttpResponse, web};
use crate::config::ProjectConfig;
use actix_session::Session;
use handlebars::Handlebars;
use crate::db_api::DbConnection;
use crate::db_api::db_result::SessionErrorType::DbError;
use crate::js_api::response_result::BackendError;
use crate::js_api::response_result::ErrorCode::{DatabaseError, Unauthorized, UserInputError, NoResult, Ignored, UnknownError, CookieError, InternalError};
use crate::security::{get_user_session};
use crate::js_api::{get_filter_ref};
use crate::frontend::template_data::UploadViewTemplateData;

pub async fn index(config: web::Data<ProjectConfig>, handlebars: web::Data<Handlebars<'_>>, session: Session) -> HttpResponse {
    let mut template_data = UploadViewTemplateData::new_empty();
    let db_connection = DbConnection::new(config.as_ref(), true, true).await;

    if db_connection.is_ok() {
        let db_connection = db_connection.ok().unwrap();
        let user_session = get_user_session(&db_connection, &session, true).await;

        if user_session.is_ok() {
            // Database connection is ok and user is logged in

            // Get filter settings
            let filter_settings = get_filter_ref(&config, &session).await;

        }
        else {
            let session_error = user_session.err().unwrap();
            let error_type = session_error.error_type;

            if error_type == DbError {
                //template_data.backend_error = true;
                //template_data.error_msg = Some(String::from("Es ist ein unerwarteter Datenbankfehler aufgetreten!"));
            }

            //template_data.user_logged_in = false;
        }
    }
    else {
        //template_data.backend_error = true;
        //template_data.user_logged_in = false;
        //template_data.error_msg = Some(String::from("Fehler beim Verbinden mit der Datenbank!"));
    }

    return HttpResponse::Ok().body("Test");
}