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

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct CommentData {
    comment_id: i32,
    comment_timestamp: DateTime<Local>,
    comment_text: String,
    comment_poster: i32,
    comment_upvotes: i32,
}

impl CommentData {
    pub fn new(comment_id: i32, comment_timestamp: DateTime<Local>, comment_text: &str,
               comment_poster: i32, comment_upvotes: i32,) -> CommentData {
        CommentData {
            comment_id,
            comment_timestamp,
            comment_text: comment_text.to_owned(),
            comment_poster,
            comment_upvotes,
        }
    }
}

#[derive(Clone)]
pub struct CommentList {
    comment_list: Vec<CommentData>,
}

impl CommentList {
    pub fn new() -> CommentList {
        CommentList {
            comment_list: Vec::new(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum DbApiErrorType {
    UnknownError,
    ConnectionError,
    ParameterError,
    QueryError,
    NoResult,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct SessionData {
    pub expire_datetime: DateTime<Local>,
    pub is_lts: bool,
    pub session_id: String,
    pub user_id: i32,
}

impl SessionData {
    pub fn new(session_id: String, user_id: i32, expire_datetime: DateTime<Local>, is_lts: bool) -> SessionData {
        SessionData {
            expire_datetime,
            is_lts,
            session_id,
            user_id,
        }
    }
}

#[derive(Copy, Clone)]
pub enum SessionErrorType {
    UnknownError,
    DbError,
    SessionExpired,
    SessionInvalid,
}

#[derive(Clone)]
pub struct SessionError {
    pub error_type: SessionErrorType,
    pub error_msg: String,
}

impl SessionError {
    pub fn new(error_type: SessionErrorType, error_msg: &str) -> SessionError {
        SessionError {
            error_type,
            error_msg: error_msg.to_owned(),
        }
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct TagData {
    pub tag_id: i32,
    pub tum_id: i32,
    pub tag_text: String,
    pub tag_poster: i32,
    pub tag_upvotes: i32,
}

impl TagData {
    pub fn new(tag_id: i32, tum_id: i32, tag_text: &str, tag_poster: i32, tag_upvotes: i32) -> TagData {
        TagData {
            tag_id,
            tum_id,
            tag_text: tag_text.to_owned(),
            tag_poster,
            tag_upvotes,
        }
    }
}

#[derive(Clone)]
pub struct TagList {
    pub tag_list: Vec<TagData>,
}

impl TagList {
    pub fn new() -> TagList {
        TagList {
            tag_list: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct UploadData {
    pub upload_id: i32,
    pub upload_is_nsfw: bool,
    pub upload_url: String,
    pub uploader: i32,
    pub upload_timestamp: DateTime<Local>,
    pub upload_upvotes: i32,
    pub tag_list: TagList,
    pub comment_list: CommentList,
}

impl UploadData {
    pub fn new(upload_id: i32, upload_is_nsfw: bool, upload_url: String, uploader: i32,
               upload_timestamp: DateTime<Local>, upload_upvotes: i32) -> UploadData {
        UploadData {
            upload_id,
            upload_is_nsfw,
            upload_url,
            uploader,
            upload_timestamp,
            upload_upvotes,
            tag_list: TagList::new(),
            comment_list: CommentList::new(),
        }
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct UploadPrvList {
    pub uploads: Vec<UploadPreview>,
}
