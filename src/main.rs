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

#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_json;

use actix_web::web;
use actix_web::{App, HttpResponse, HttpServer};
use actix_files as fs;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/", "./static/webcontent/").index_file("index.html"))
            .service(fs::Files::new("/uploads", "./static/uploads/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
