use crate::db_api::db_result::{UploadData, UploadPreview};
use crate::backend_api::response_result::{UserData, Filter, BackendError};
use actix_web::web;
use actix_session::Session;
use crate::config::ProjectConfig;
use serde::{Serialize};
use crate::backend_api::{get_filter, get_own_userdata};
use crate::security::get_user_session;

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

// Main struct
#[derive(Clone, Serialize)]
pub struct IndexViewTemplateData {
    pub backend_data: BackendData,
    pub session_settings: Option<Filter>,
    pub uploads_prv_list: Vec<UploadPreview>,
    pub user_data: Option<UserData>,
}

impl IndexViewTemplateData {
    // Used if the user is not logged in
    pub fn new_empty() -> IndexViewTemplateData {
        IndexViewTemplateData {
            backend_data: BackendData::new_default(),
            session_settings: None,
            uploads_prv_list: Vec::new(),
            user_data: None,
        }
    }

    // Used for backend errors (for example if the database is offline)
    pub fn new_error(error_msg: &str) -> IndexViewTemplateData {
        IndexViewTemplateData {
            backend_data: BackendData::new(true, error_msg),
            session_settings: None,
            uploads_prv_list: Vec::new(),
            user_data: None,
        }
    }

    // Generate template data for the index view
    pub async fn new_index(config: web::Data<ProjectConfig>, session: Session) -> IndexViewTemplateData {
        let filter_data = get_filter(&config, &session).await;

        if filter_data.is_err() {
            let backend_error = filter_data.err().unwrap();

            return IndexViewTemplateData::new_error(backend_error.error_msg.as_str());
        }

        let user_data = get_own_userdata(&config, &session);

        return IndexViewTemplateData::new_error("Es ist ein unbekannter Fehler aufgetreten");
    }
}