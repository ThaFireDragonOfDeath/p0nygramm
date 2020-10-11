use crate::db_api::db_result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum ErrorCode {
    DatabaseError,
    UserInputError,
    NoResult,
    Unauthorized,
    Ignored,
    UnknownError,
    CookieError,
    InternalError,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BackendError {
    pub http_status_code: u16,
    pub error_code: ErrorCode,
    pub error_msg: String,
}

impl BackendError {
    pub fn new(http_status_code: u16, error_code: ErrorCode, error_msg: &str) -> BackendError {
        BackendError {
            http_status_code,
            error_code,
            error_msg: error_msg.to_owned(),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct AddUploadSuccess {
    upload_success: bool,
    upload_id: i32,
    tags_part_success: bool,
    tags_full_success: bool,
}

impl AddUploadSuccess {
    pub fn new(upload_success: bool, upload_id: i32, tags_part_success: bool, tags_full_success: bool) -> AddUploadSuccess {
        AddUploadSuccess {
            upload_success,
            upload_id,
            tags_part_success,
            tags_full_success,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct UserExists {
    user_exists: bool,
}

impl UserExists {
    pub fn new(user_exists: bool) -> UserExists {
        UserExists {
            user_exists,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct Filter {
    pub show_sfw: bool,
    pub show_nsfw: bool,
}

impl Filter {
    pub fn new(show_sfw: bool, show_nsfw: bool) -> Filter {
        Filter {
            show_sfw,
            show_nsfw,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct SuccessReport {
    pub success: bool,
}

impl SuccessReport {
    pub fn new(success: bool) -> SuccessReport {
        SuccessReport {
            success,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct UserData {
    pub user_id: i32,
    pub username: String,
    pub user_is_mod: bool,
}

impl UserData {
    pub fn new(db_userdata: &db_result::UserData) -> UserData {
        UserData {
            user_id: db_userdata.user_id,
            username: db_userdata.username.clone(),
            user_is_mod: db_userdata.user_is_mod,
        }
    }
}