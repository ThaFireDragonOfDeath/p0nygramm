use crate::db_api::db_result::{UploadPrvList, UploadData};
use crate::backend_api::response_result::{UserData, Filter, BackendError};
use actix_web::web;
use actix_session::Session;
use crate::config::ProjectConfig;
use serde::{Serialize};
use crate::backend_api::{get_filter, get_own_userdata, get_uploads};
use crate::backend_api::response_result::ErrorCode::{Unauthorized};

const INDEX_START_AMOUNT: i16 = 50;

// Main struct
#[derive(Clone, Serialize)]
pub struct IndexViewTemplateData {
    pub backend_error: Option<BackendError>,
    pub filter_settings: Option<Filter>,
    pub uploads_prv_list: Option<UploadPrvList>,
    pub upload_data: Option<UploadData>,
    pub user_data: Option<UserData>,
}

impl IndexViewTemplateData {
    // Used if the user is not logged in
    pub fn new_empty() -> IndexViewTemplateData {
        IndexViewTemplateData {
            backend_error: None,
            filter_settings: None,
            uploads_prv_list: None,
            upload_data: None,
            user_data: None,
        }
    }

    // Used for backend errors (for example if the database is offline)
    pub fn new_error(backend_error: BackendError) -> IndexViewTemplateData {
        IndexViewTemplateData {
            backend_error: Some(backend_error),
            filter_settings: None,
            uploads_prv_list: None,
            upload_data: None,
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

        let filter_data = filter_data.ok().unwrap();

        let start_id = i32::max_value();
        let show_sfw = filter_data.show_sfw;
        let show_nsfw = filter_data.show_nsfw;
        let url_data = web::Path::from((start_id, INDEX_START_AMOUNT, show_sfw, show_nsfw));
        let uploads_prv = get_uploads(&config, &session, &url_data).await;

        if uploads_prv.is_err() {
            let backend_error = uploads_prv.err().unwrap();

            return IndexViewTemplateData::new_error(backend_error);
        }

        let user_data = get_own_userdata(&config, &session).await;

        if user_data.is_err() {
            let backend_error = user_data.err().unwrap();

            return IndexViewTemplateData::new_error(backend_error);
        }

        let uploads_prv = uploads_prv.ok().unwrap();
        let user_data = user_data.ok().unwrap();

        IndexViewTemplateData {
            backend_error: None,
            filter_settings: Some(filter_data),
            uploads_prv_list: Some(uploads_prv),
            upload_data: None,
            user_data: Some(user_data)
        }
    }
}