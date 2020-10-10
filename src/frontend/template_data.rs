use crate::db_api::db_result::{UploadData, UploadPreview};
use crate::backend_api::response_result::UserData;
use actix_web::web;
use actix_session::Session;
use crate::config::ProjectConfig;
use serde::{Serialize};

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
    // Used if the user is not logged in
    pub fn new_empty() -> UploadViewTemplateData {
        UploadViewTemplateData {
            backend_data: BackendData::new_default(),
            session_settings: None,
            uploads_prv: Vec::new(),
            user_data: None,
            upload_data: None,
        }
    }

    // Used for backend errors (for example if the database is offline)
    pub fn new_error(error_msg: &str) -> UploadViewTemplateData {
        UploadViewTemplateData {
            backend_data: BackendData::new(true, error_msg),
            session_settings: None,
            uploads_prv: Vec::new(),
            user_data: None,
            upload_data: None,
        }
    }

    // Generate template data for the index view
    pub async fn new_index(config: web::Data<ProjectConfig>, session: Session) -> UploadViewTemplateData {
        return UploadViewTemplateData::new_error("Es ist ein unbekannter Fehler aufgetreten");
    }
}