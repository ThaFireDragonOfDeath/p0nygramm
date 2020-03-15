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

mod config;
mod db_api;
mod file_api;
mod js_api;
mod security;

#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_json;

extern crate argonautica;
extern crate chrono;
extern crate log;
extern crate rand;
extern crate tokio;

use actix_web::web;
use actix_web::{App, HttpResponse, HttpServer};
use actix_files as fs;
use crate::config::ProjectConfig;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let prj_config = ProjectConfig::init();

    if prj_config.is_some() {
        let prj_config = prj_config.unwrap();
        let prj_config_data = web::Data::new(prj_config);

        HttpServer::new(move || {
            App::new()
                .service(fs::Files::new("/", "./static/webcontent/").index_file("index.html"))
                .service(fs::Files::new("/uploads", "./static/uploads/").index_file("index.html"))
                .service(fs::Files::new("/prv", "./static/uploads-prv/").index_file("index.html"))
                .service(web::scope("/js-api")
                    .app_data(prj_config_data.clone())
                    .route("/get_upload_data/{upload_id}", web::get().to(js_api::get_upload_data))
                    .route("/logout", web::get().to(js_api::logout))
                )
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
