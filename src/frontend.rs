pub mod template_data;

use actix_web::{HttpResponse, web};
use crate::config::ProjectConfig;
use actix_session::Session;
use handlebars::Handlebars;

pub async fn index(config: web::Data<ProjectConfig>, handlebars: web::Data<Handlebars<'_>>, session: Session) -> HttpResponse {
    return HttpResponse::Ok().body("Test");
}