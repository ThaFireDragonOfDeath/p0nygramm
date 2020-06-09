use crate::db_api::db_result::{UploadData, UploadPreview};
use crate::js_api::response_result::UserData;

// Helper struct
#[derive(Clone, Serialize)]
pub struct BackendData {
    pub backend_error: bool,
    pub error_msg: Option<String>,
}

impl BackendData {
    pub fn new_default() -> BackendData {
        BackendData {
            backend_error: false,
            error_msg: None,
        }
    }

    pub fn new(backend_error: bool, error_msg: &str) -> BackendData {
        BackendData {
            backend_error,
            error_msg: Some(error_msg.to_owned()),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct SessionSettings {
    pub show_sfw: bool,
    pub show_nsfw: bool,
}

// Main struct
#[derive(Clone, Serialize)]
pub struct UploadViewTemplateData {
    pub backend_data: BackendData,
    pub session_settings: Option<SessionSettings>,
    pub uploads_prv: Vec<UploadPreview>,
    pub user_data: Option<UserData>,
    pub upload_data: Option<UploadData>,
}

impl UploadViewTemplateData {
    pub fn new_empty() -> UploadViewTemplateData {
        UploadViewTemplateData {
            backend_data: BackendData::new_default(),
            session_settings: None,
            uploads_prv: Vec::new(),
            user_data: None,
            upload_data: None,
        }
    }
}