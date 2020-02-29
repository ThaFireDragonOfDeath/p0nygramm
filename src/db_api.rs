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
use crate::db_api::DbApiErrorType::{UnknownError, ConnectionError};
use crate::file_api::{get_preview_url_from_filename, get_url_from_filename};

mod postgres;
mod redis;

macro_rules! check_postgres_connection {
    ($self:ident) => {
        if $self.postgres_connection.is_none() {
            return Err(DbApiError::new(ConnectionError, "Keine Verbindung zum Postgres Server vorhanden!"));
        }
    };
}

pub enum DbApiErrorType {
    UnknownError,
    ConnectionError,
    ParameterError,
    QueryError,
    NoResult,
}

// Result structures
pub struct DbApiError {
    pub error_type: DbApiErrorType,
    pub error_msg: String,
}

impl DbApiError {
    pub fn new(error_type: DbApiErrorType, error_msg: &str) -> DbApiError {
        DbApiError {
            error_type,
            error_msg: error_msg.to_owned(),
        }
    }
}

pub struct UploadPreview {
    pub upload_id: i32,
    pub upload_is_nsfw: bool,
    pub upload_prv_url: String,
    pub upload_url: String,
}

impl UploadPreview {
    pub fn new(upload_id: i32, upload_is_nsfw: bool, upload_filename: String) -> UploadPreview {
        UploadPreview {
            upload_id,
            upload_is_nsfw,
            upload_prv_url: get_preview_url_from_filename(upload_filename.as_str()),
            upload_url: get_url_from_filename(upload_filename.as_str()),
        }
    }
}

pub struct UploadPrvList {
    pub uploads: Vec<UploadPreview>,
}

pub struct DbConnection {
    postgres_connection: Option<PostgresConnection>,
    redis_connection: Option<RedisConnection>,
}

impl DbConnection {
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

    pub async fn new(project_config: &ProjectConfig) -> DbConnection {
        DbConnection {
            postgres_connection: PostgresConnection::new(project_config).await,
            redis_connection: RedisConnection::new(project_config).await,
        }
    }

    pub fn search_uploads(&self, search_string: &str, start_id: i32, amount: i16, show_nsfw: bool) -> Result<UploadPrvList, DbApiError> {
        Err(DbApiError::new(UnknownError, "Unbekannter Fehler"))
    }
}