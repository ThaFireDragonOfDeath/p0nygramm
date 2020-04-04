use actix_web::{HttpResponse, web};
use crate::config::ProjectConfig;
use actix_session::Session;
use handlebars::Handlebars;
use crate::db_api::DbConnection;
use crate::js_api::response_result::BackendError;
use crate::js_api::response_result::ErrorCode::{DatabaseError, Unauthorized, UserInputError, NoResult, Ignored, UnknownError, CookieError, InternalError};

pub async fn index(config: web::Data<ProjectConfig>, handlebars: web::Data<Handlebars<'_>>, session: Session) -> HttpResponse {
    let db_connection = get_db_connection!(config, true, true);

    return HttpResponse::Ok().body("Test");
}