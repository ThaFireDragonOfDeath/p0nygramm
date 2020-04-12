use crate::db_api::db_result;

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

#[derive(Serialize, Deserialize)]
pub struct BackendError {
    error_code: ErrorCode,
    error_msg: String,
}

impl BackendError {
    pub fn new(error_code: ErrorCode, error_msg: &str) -> BackendError {
        BackendError {
            error_code,
            error_msg: error_msg.to_owned(),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct AddUploadSuccess {
    upload_success: bool,
    tags_part_success: bool,
    tags_full_success: bool,
}

impl AddUploadSuccess {
    pub fn new(upload_success: bool, tags_part_success: bool, tags_full_success: bool) -> AddUploadSuccess {
        AddUploadSuccess {
            upload_success,
            tags_part_success,
            tags_full_success,
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