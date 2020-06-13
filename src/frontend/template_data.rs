use crate::db_api::db_result::{UploadData, UploadPreview};
use crate::js_api::response_result::UserData;
use actix_web::web;
use actix_session::Session;
use crate::config::ProjectConfig;
use crate::db_api::DbConnection;
use crate::security::get_user_session;
use crate::js_api::get_filter_ref;
use crate::db_api::db_result::SessionErrorType::{DbError, SessionExpired, SessionInvalid, NoSession};

// Macros
macro_rules! get_db_connection {
    ($config:ident, $req_postgres:expr, $req_redis:expr) => {
        {
            let db_connection = DbConnection::new($config.as_ref(), $req_postgres, $req_redis).await;

            if db_connection.is_err() {
                handle_db_connection_error!(db_connection);
            }

            db_connection.ok().unwrap()
        }
    };
}

macro_rules! get_user_session_data {
    ($db_connection:ident, $session:ident, $force_session_renew:expr) => {
        {
            let user_session = get_user_session(&$db_connection, &$session, $force_session_renew).await;

            if user_session.is_err() {
                handle_session_error!(user_session);
            }

            user_session.ok().unwrap()
        }
    };
}

macro_rules! handle_db_connection_error {
    ($db_connection:ident) => {
        let error = $db_connection.err().unwrap();
        let error_txt = error.error_msg;

        handle_error_str!(error_txt.as_str());
    };
}

macro_rules! handle_error_str {
    ($error_str:expr) => {
        return UploadViewTemplateData::new_error($error_str);
    };
}

macro_rules! handle_session_error {
    ($user_session:ident) => {
        let error = $user_session.err().unwrap();
        let error_type = error.error_type;
        let error_txt = error.error_msg;

        // If there is no valid session
        if error_type == SessionExpired || error_type == SessionInvalid || error_type == NoSession {
            return UploadViewTemplateData::new_empty();
        }

        handle_error_str!(error_txt.as_str());
    };
}

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
    pub async fn new_index(config: &web::Data<ProjectConfig>, session: &Session) -> UploadViewTemplateData {
        let db_connection = get_db_connection!(config, true, true);
        let user_session = get_user_session_data!(db_connection, session, false);

        let current_user_id = user_session.user_id;

        return UploadViewTemplateData::new_error("Es ist ein unbekannter Fehler aufgetreten");
    }
}