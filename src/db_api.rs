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
use crate::db_api::result::{UploadPrvList, DbApiError, SessionData, SessionError, UploadData};
use crate::db_api::result::DbApiErrorType::{UnknownError, ConnectionError};
use crate::db_api::result::SessionErrorType::DbError;

mod postgres;
mod redis;
mod result;

pub struct DbConnection {
    postgres_connection: Option<PostgresConnection>,
    redis_connection: Option<RedisConnection>,
}

impl DbConnection {
    pub async fn add_comment(&self, comment_poster: i32, comment_upload: i32, comment_text: &str) -> Result<(), DbApiError> {
        if !self.have_postgres_connection() {
            return Err(DbApiError::new(ConnectionError, "Keine Verbindung zum Postgres Server vorhanden!"));
        }

        return self.postgres_connection.as_ref().unwrap().add_comment(comment_poster, comment_upload, comment_text).await;
    }

    // Returns the upload_id of the new inserted upload or error
    pub async fn add_upload(&self, upload_filename: &str, upload_is_nsfw: bool, uploader: i32) -> Result<i32, DbApiError> {
        if !self.have_postgres_connection() {
            return Err(DbApiError::new(ConnectionError, "Keine Verbindung zum Postgres Server vorhanden!"));
        }

        return self.postgres_connection.as_ref().unwrap().add_upload(upload_filename, upload_is_nsfw, uploader).await;
    }

    pub async fn get_session_data(&self, session_id: &str, force_session_renew: bool) -> Result<SessionData, SessionError> {
        if !self.have_redis_connection() {
            return Err(SessionError::new(DbError, "Fehler beim Zugriff auf die Redis Datenbank"));
        }

        let redis_connection = self.redis_connection.as_ref().unwrap();
        let session_data = redis_connection.get_session_data(session_id).await;

        if session_data.is_ok() {
            let session_data = session_data.as_ref().ok().unwrap();

            redis_connection.renew_session(session_data, force_session_renew).await;
        }

        return session_data;
    }

    pub async fn get_upload_data(&self, upload_id: i32) -> Result<UploadData, DbApiError> {
        if !self.have_postgres_connection() {
            return Err(DbApiError::new(ConnectionError, "Keine Verbindung zum Postgres Server vorhanden!"));
        }

        return self.postgres_connection.as_ref().unwrap().get_upload_data(upload_id).await;
    }

    pub async fn get_uploads(&self, start_id: i32, max_count: i16, show_nsfw: bool) -> Result<UploadPrvList, DbApiError> {
        if !self.have_postgres_connection() {
            return Err(DbApiError::new(ConnectionError, "Keine Verbindung zum Postgres Server vorhanden!"));
        }

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
        // TODO: Implement

        Err(DbApiError::new(UnknownError, "Unbekannter Fehler"))
    }
}