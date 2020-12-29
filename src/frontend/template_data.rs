use crate::backend_api::response_result::{UserData, Filter, BackendError};
use actix_web::web;
use actix_session::Session;
use crate::config::ProjectConfig;
use serde::{Serialize};
use crate::backend_api::{get_filter, get_own_userdata};
use crate::backend_api::response_result::ErrorCode::{Unauthorized};

// Main struct
#[derive(Clone, Serialize)]
pub struct IndexViewTemplateData {
    pub backend_error: Option<BackendError>,
    pub filter_settings: Option<Filter>,
    pub user_data: Option<UserData>,
    pub read_access: bool,
}

impl IndexViewTemplateData {
    // Used if the user is not logged in
    pub fn new_empty() -> IndexViewTemplateData {
        IndexViewTemplateData {
            backend_error: None,
            filter_settings: None,
            user_data: None,
            read_access: false,
        }
    }

    // Used for backend errors (for example if the database is offline)
    pub fn new_error(backend_error: BackendError) -> IndexViewTemplateData {
        IndexViewTemplateData {
            backend_error: Some(backend_error),
            filter_settings: None,
            user_data: None,
            read_access: false,
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

        let filter_data = filter_data.ok().unwrap();
        let user_data = get_own_userdata(&config, &session).await;

        if user_data.is_err() {
            let backend_error = user_data.err().unwrap();

            return IndexViewTemplateData::new_error(backend_error);
        }

        let user_data = user_data.ok().unwrap();

        IndexViewTemplateData {
            backend_error: None,
            filter_settings: Some(filter_data),
            user_data: Some(user_data),
            read_access: true,
        }
    }
}