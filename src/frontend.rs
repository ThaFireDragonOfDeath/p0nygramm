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