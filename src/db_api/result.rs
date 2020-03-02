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

use crate::file_api::{get_preview_url_from_filename, get_url_from_filename};
use chrono::{DateTime, Local};

pub enum DbApiErrorType {
    UnknownError,
    ConnectionError,
    ParameterError,
    QueryError,
    NoResult,
}

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

pub struct SessionData {
    pub session_id: String,
    pub user_id: i32,
    pub expire_datetime: DateTime<Local>,
}

impl SessionData {
    pub fn new(session_id: String, user_id: i32, expire_datetime: DateTime<Local>) -> SessionData {
        SessionData {
            session_id,
            user_id,
            expire_datetime,
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
