use crate::db_api::db_result::{UploadData, UploadPreview};
use crate::backend_api::response_result::{UserData, Filter, BackendError};
use actix_web::web;
use actix_session::Session;
use crate::config::ProjectConfig;
use serde::{Serialize};
use crate::backend_api::{get_filter, get_own_userdata};
use crate::security::get_user_session;
use crate::backend_api::response_result::ErrorCode::{UnknownError, Unauthorized};
use actix_web::http::StatusCode;

// Main struct
#[derive(Clone, Serialize)]
pub struct IndexViewTemplateData {
    pub backend_error: Option<BackendError>,
    pub session_settings: Option<Filter>,
    pub uploads_prv_list: Vec<UploadPreview>,
    pub user_data: Option<UserData>,
}

impl IndexViewTemplateData {
    // Used if the user is not logged in
    pub fn new_empty() -> IndexViewTemplateData {
        IndexViewTemplateData {
            backend_error: None,
            session_settings: None,
            uploads_prv_list: Vec::new(),
            user_data: None,
        }
    }

    // Used for backend errors (for example if the database is offline)
    pub fn new_error(backend_error: BackendError) -> IndexViewTemplateData {
        IndexViewTemplateData {
            backend_error: Some(backend_error),
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

            // Return empty data object if the user isn't logged in
            if backend_error.error_code == Unauthorized {
                return IndexViewTemplateData::new_empty();
            }

            return IndexViewTemplateData::new_error(backend_error);
        }

        let user_data = get_own_userdata(&config, &session).await;

        if user_data.is_err() {
            let backend_error = filter_data.err().unwrap();

            return IndexViewTemplateData::new_error(backend_error);
        }

        // Dummy
        let backend_error = BackendError::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), UnknownError, "Implementation ist noch nicht fertiggestellt");
        return IndexViewTemplateData::new_error(backend_error);
    }
}