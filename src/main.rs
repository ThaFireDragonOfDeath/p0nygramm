mod js_api;
mod config;
mod db_api;
mod file_api;
mod frontend;
mod security;
mod backend_api;

use actix_web::web;
use actix_web::{App, HttpServer};
use actix_files as fs;
use crate::config::ProjectConfig;
use actix_session::CookieSession;
use handlebars::Handlebars;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let prj_config = ProjectConfig::init();

    if prj_config.is_some() {
        let prj_config = prj_config.unwrap();
        let session_private_key = prj_config.security_config.session_private_key.get_value();
        let template_path = prj_config.filesystem_config.template_path.get_value();
        let prj_config_data = web::Data::new(prj_config);

        let mut handlebars = Handlebars::new();

        handlebars
            .register_templates_directory(".html", template_path.as_str())
            .unwrap();
        let handlebars_data = web::Data::new(handlebars);

        HttpServer::new(move || {
            App::new()
                .service(
                    web::scope("/")
                    .wrap(CookieSession::signed(session_private_key.as_bytes())
                        .http_only(true)
                        .max_age(2592000) // 30 days
                        .name("session_data")
                        .path("/")
                        .secure(true)
                    )
                    .app_data(prj_config_data.clone())
                    .app_data(handlebars_data.clone())
                    .route("/", web::get().to(frontend::index))
                )
                .service(
                    web::scope("/js-api")
                    .wrap(CookieSession::signed(session_private_key.as_bytes())
                        .http_only(true)
                        .max_age(2592000) // 30 days
                        .name("session_data")
                        .path("/")
                        .secure(true)
                    )
                    .app_data(prj_config_data.clone())
                    .route("/add_comment", web::post().to(js_api::add_comment))
                    .route("/add_upload", web::post().to(js_api::add_upload))
                    .route("/check_username_exists/{username}", web::get().to(js_api::check_username_exists))
                    .route("/get_filter", web::get().to(js_api::get_filter))
                    .route("/get_uploads/{start_id}/{amount}/{show_sfw}/{show_nsfw}", web::get().to(js_api::get_uploads))
                    .route("/get_uploads_range/{start_id}/{amount}/{show_sfw}/{show_nsfw}", web::get().to(js_api::get_uploads_range))
                    .route("/get_upload_data/{upload_id}", web::get().to(js_api::get_upload_data))
                    .route("/get_userdata_by_username/{username}", web::get().to(js_api::get_userdata_by_username))
                    .route("/login", web::post().to(js_api::login))
                    .route("/logout", web::get().to(js_api::logout))
                    .route("/register", web::post().to(js_api::register))
                    .route("/set_filter/{show_sfw}/{show_nsfw}", web::get().to(js_api::set_filter))
                    .route("/vote_comment/{comment_id}/{vote_value}", web::get().to(js_api::vote_comment))
                    .route("/vote_tag/{tum_id}/{vote_value}", web::get().to(js_api::vote_tag))
                    .route("/vote_upload/{upload_id}/{vote_value}", web::get().to(js_api::vote_upload))
                )
                .service(fs::Files::new("/uploads", "./static/uploads/").index_file("index.html"))
                .service(fs::Files::new("/prv", "./static/uploads-prv/").index_file("index.html"))
                .service(fs::Files::new("/static", "./static/webcontent/").index_file("index.html"))
        })
            .bind("127.0.0.1:8080")?
            .run()
            .await
    }
    else {
        let error = std::io::Error::new(std::io::ErrorKind::Other, "Failed to read config");
        return Err(error);
    }
}
