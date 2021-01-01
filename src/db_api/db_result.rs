use crate::file_api::{get_preview_url_from_filename, get_url_from_filename};
use chrono::{DateTime, Local};
use serde::{Serialize};
use postgres_types::{ToSql, FromSql};
use crate::db_api::db_result::UploadType::{AnimatedImage, Image, Video};

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CommentData {
    comment_timestamp: DateTime<Local>,
    comment_text: String,
    comment_poster_id: i32,
    comment_poster_username : String,
    comment_upvotes: i32,
}

impl CommentData {
    pub fn new(comment_timestamp: DateTime<Local>, comment_text: &str, comment_poster_id: i32,
               comment_poster_username: &str, comment_upvotes: i32,) -> CommentData {
        CommentData {
            comment_timestamp,
            comment_text: comment_text.to_owned(),
            comment_poster_id,
            comment_poster_username: comment_poster_username.to_owned(),
            comment_upvotes,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct CommentList {
    comment_list: Vec<CommentData>,
}

impl CommentList {
    pub fn new() -> CommentList {
        CommentList {
            comment_list: Vec::new(),
        }
    }

    pub fn add_comment(&mut self, comment_data: CommentData) {
        self.comment_list.push(comment_data);
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DbApiErrorType {
    ConnectionError,
    QueryError,
    NoResult,
    PartFail,
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

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SessionErrorType {
    UnknownError,
    DbError,
    SessionInvalid,
    NoSession,
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

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TagData {
    pub tag_id: i32,
    pub tag_text: String,
    pub tag_upvotes: i32,
}

impl TagData {
    pub fn new(tag_id: i32, tag_text: &str, tag_upvotes: i32) -> TagData {
        TagData {
            tag_id,
            tag_text: tag_text.to_owned(),
            tag_upvotes,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct TagList {
    pub tag_list: Vec<TagData>,
}

impl TagList {
    pub fn new() -> TagList {
        TagList {
            tag_list: Vec::new(),
        }
    }

    pub fn add_tag(&mut self, tag_data: TagData) {
        self.tag_list.push(tag_data);
    }
}

#[derive(Clone, Serialize, Eq, PartialEq, Debug, ToSql, FromSql)]
pub enum UploadType {
    Image,
    AnimatedImage,
    Video,
}

impl UploadType {
    pub fn new(is_image: bool, is_animated_image: bool, is_video: bool) -> Option<UploadType> {
        if is_image {
            if is_animated_image {
                Some(AnimatedImage)
            }
            else {
                Some(Image)
            }
        }
        else if is_video {
            Some(Video)
        }
        else {
            None
        }
    }
}

#[derive(Clone, Serialize)]
pub struct UploadData {
    pub upload_id: i32,
    pub upload_is_sfw: bool,
    pub upload_is_nsfw: bool,
    pub upload_type: UploadType,
    pub upload_url: String,
    pub uploader_id: i32,
    pub uploader_username: String,
    pub upload_timestamp: DateTime<Local>,
    pub upload_upvotes: i32,
    pub tag_list: TagList,
    pub comment_list: CommentList,
}

impl UploadData {
    pub fn new(upload_id: i32, upload_is_nsfw: bool, upload_type: UploadType, upload_filename: &str, uploader_id: i32,
               uploader_username: &str, upload_timestamp: DateTime<Local>, upload_upvotes: i32) -> UploadData {
        UploadData {
            upload_id,
            upload_is_sfw: !upload_is_nsfw,
            upload_is_nsfw,
            upload_type,
            upload_url: get_url_from_filename(upload_filename),
            uploader_id,
            uploader_username: uploader_username.to_owned(),
            upload_timestamp,
            upload_upvotes,
            tag_list: TagList::new(),
            comment_list: CommentList::new(),
        }
    }

    pub fn add_comment(&mut self, comment_timestamp: DateTime<Local>, comment_text: &str,
                       comment_poster_id: i32, comment_poster_username: &str, comment_upvotes: i32) {
        let comment_data = CommentData::new(comment_timestamp, comment_text,
                                            comment_poster_id, comment_poster_username, comment_upvotes);

        self.comment_list.add_comment(comment_data);
    }

    pub fn add_tag(&mut self, tag_id: i32, tag_text: &str, tag_upvotes: i32) {
        let tag_data = TagData::new(tag_id, tag_text, tag_upvotes);

        self.tag_list.add_tag(tag_data);
    }
}

#[derive(Clone, Serialize)]
pub struct UploadPreview {
    pub upload_id: i32,
    pub upload_is_nsfw: bool,
    pub upload_prv_url: String,
    pub upload_url: String,
    pub upload_type: UploadType,
}

impl UploadPreview {
    pub fn new(upload_id: i32, upload_is_nsfw: bool, upload_filename: String, upload_type: UploadType) -> UploadPreview {
        UploadPreview {
            upload_id,
            upload_is_nsfw,
            upload_prv_url: get_preview_url_from_filename(upload_filename.as_str()),
            upload_url: get_url_from_filename(upload_filename.as_str()),
            upload_type,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct UploadPrvList {
    pub uploads: Vec<UploadPreview>,
}

#[derive(Clone)]
pub struct UserData {
    pub user_id: i32,
    pub username: String,
    pub password_hash: String,
    pub user_is_mod: bool,
}

impl UserData {
    pub fn new(user_id: i32, username: &str, password_hash: &str, user_is_mod: bool) -> UserData {
        UserData {
            user_id,
            username: username.to_owned(),
            password_hash: password_hash.to_owned(),
            user_is_mod,
        }
    }
}