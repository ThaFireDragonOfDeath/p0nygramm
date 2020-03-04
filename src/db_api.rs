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

use crate::db_api::postgres::PostgresConnection;
use crate::db_api::redis::RedisConnection;
use crate::config::ProjectConfig;
use crate::file_api::{get_preview_url_from_filename, get_url_from_filename};
use crate::db_api::result::{UploadPrvList, DbApiError, SessionData};
use crate::db_api::result::DbApiErrorType::{UnknownError, ConnectionError};

mod postgres;
mod redis;
mod result;

macro_rules! check_postgres_connection {
    ($self:ident) => {
        if $self.have_postgres_connection() {
            return Err(DbApiError::new(ConnectionError, "Keine Verbindung zum Postgres Server vorhanden!"));
        }
    };
}

macro_rules! check_redis_connection {
    ($self:ident) => {
        if $self.have_redis_connection() {
            return Err(DbApiError::new(ConnectionError, "Keine Verbindung zum Redis Server vorhanden!"));
        }
    };
}

pub struct DbConnection {
    postgres_connection: Option<PostgresConnection>,
    redis_connection: Option<RedisConnection>,
}

impl DbConnection {
    pub async fn get_session_data(&self, session_id: &str, force_session_renew: bool) -> Result<SessionData, DbApiError> {
        check_redis_connection!(self);
        let redis_connection = self.redis_connection.as_ref().unwrap();
        let session_data = redis_connection.get_session_data(session_id).await;

        if session_data.is_ok() {
            let session_data = session_data.as_ref().ok().unwrap();

            redis_connection.renew_session(session_data, force_session_renew).await;
        }

        return session_data;
    }

    pub async fn get_uploads(&self, start_id: i32, max_count: i16, show_nsfw: bool) -> Result<UploadPrvList, DbApiError> {
        check_postgres_connection!(self);

        return self.postgres_connection.as_ref().unwrap().get_uploads(start_id, max_count, show_nsfw).await;
    }

    pub fn have_postgres_connection(&self) -> bool {
        self.postgres_connection.is_some()
    }

    pub fn have_redis_connection(&self) -> bool {
        self.redis_connection.is_some()
    }

    pub async fn new(project_config: &ProjectConfig, require_postgres: bool, require_redis: bool) -> Result<DbConnection, DbApiError> {
        let postgres_connection = PostgresConnection::new(project_config).await;
        let redis_connection = RedisConnection::new(project_config).await;

        if postgres_connection.is_none() && require_postgres {
            return Err(DbApiError::new(ConnectionError, "Keine Verbindung zum Postgres Server vorhanden!"));
        }
        else if redis_connection.is_none() && require_redis {
            return Err(DbApiError::new(ConnectionError, "Keine Verbindung zum Redis Server vorhanden!"));
        }

        let db_connection = DbConnection {
            postgres_connection,
            redis_connection,
        };

        return Ok(db_connection);
    }

    pub fn search_uploads(&self, search_string: &str, start_id: i32, amount: i16, show_nsfw: bool) -> Result<UploadPrvList, DbApiError> {
        Err(DbApiError::new(UnknownError, "Unbekannter Fehler"))
    }
}