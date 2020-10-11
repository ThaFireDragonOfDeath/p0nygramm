pub mod template_data;

use actix_web::{HttpResponse, web};
use crate::config::ProjectConfig;
use actix_session::Session;
use handlebars::Handlebars;
use crate::frontend::template_data::IndexViewTemplateData;

pub const HANDLEBARS_ERROR_RESP: &str = "Fehler beim Generieren der Webseite (Fehler beim Rendern mit Handlebars)";

pub async fn index(config: web::Data<ProjectConfig>, handlebars: web::Data<Handlebars<'_>>, session: Session) -> HttpResponse {
    let index_view = IndexViewTemplateData::new_index(config, session).await;
    let resp_body = handlebars.render("index", &index_view);

    if resp_body.is_err() {
        return HttpResponse::InternalServerError().body(HANDLEBARS_ERROR_RESP);
    }

    return HttpResponse::Ok().body(resp_body.unwrap());
}