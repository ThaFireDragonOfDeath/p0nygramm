use crate::db_api::postgres::PostgresConnection;
use crate::db_api::redis::RedisConnection;
use crate::config::ProjectConfig;
use crate::db_api::db_result::{UploadPrvList, DbApiError, SessionData, SessionError, UploadData, UserData, UploadType};
use crate::db_api::db_result::DbApiErrorType::{ConnectionError, PartFail, QueryError, NoResult};
use crate::db_api::db_result::SessionErrorType::DbError;
use log::{trace};
use actix_session::Session;

mod postgres;
mod redis;
pub mod db_result;

macro_rules! check_postgres_connection {
    ($self:ident) => {
        if !$self.have_postgres_connection() {
            return Err(DbApiError::new(ConnectionError, "Keine Verbindung zum Postgres Server vorhanden!"));
        }
    };
}

macro_rules! check_redis_connection {
    ($self:ident) => {
        if !$self.have_redis_connection() {
            return Err(SessionError::new(DbError, "Keine Verbindung zum Redis Server vorhanden!"));
        }
    };
}

pub struct DbConnection {
    postgres_connection: Option<PostgresConnection>,
    redis_connection: Option<RedisConnection>,
}

impl DbConnection {
    pub async fn add_comment(&self, comment_poster: i32, comment_upload: i32, comment_text: &str) -> Result<(), DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().add_comment(comment_poster, comment_upload, comment_text).await
    }

    pub async fn add_tags(&self, tags: Vec<&str>, tag_poster: i32, upload_id: i32) -> Result<(), DbApiError> {
        check_postgres_connection!(self);

        let mut part_fail = false;
        let mut full_fail = true;

        for tag in tags {
            let postgres_result = self.postgres_connection.as_ref()
                .unwrap().add_tag(tag, tag_poster, upload_id).await;

            if postgres_result.is_ok() && full_fail {
                full_fail = false;
            }
            else if !part_fail {
                part_fail = true;
            }
        }

        if part_fail {
            return Err(DbApiError::new(PartFail, "Datenbank Fehler: Ein oder mehrere Tags konnten nicht hinzugefügt werden!"));
        }
        else if full_fail {
            return Err(DbApiError::new(QueryError, "Fehler beim Ausführen der SQL Anweisung"));
        }
        else {
            return Ok(());
        }
    }

    // Returns the upload_id of the new inserted upload or error
    pub async fn add_upload(&self, upload_filename: &str, upload_is_nsfw: bool, upload_type: UploadType, uploader: i32) -> Result<i32, DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().add_upload(upload_filename, upload_is_nsfw, upload_type, uploader).await
    }

    // Returns the id of the new created user
    pub async fn add_user(&self, username: &str, pw_hash: &str, user_is_mod: bool) -> Result<i32, DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().add_user(username, pw_hash, user_is_mod).await
    }

    pub async fn change_user_pw_by_username(&self, username: &str, pw_hash: &str) -> Result<(), DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().change_user_pw_by_username(username, pw_hash).await
    }

    pub async fn check_user_exists(&self, username: &str) -> Result<bool, DbApiError> {
        check_postgres_connection!(self);

        let user_data = self.postgres_connection.as_ref().unwrap().get_userdata_by_username(username).await;

        if user_data.is_ok() {
            return Ok(true);
        }
        else {
            let user_data_error = user_data.err().unwrap();

            if user_data_error.error_type == NoResult {
                return Ok(false);
            }
        }

        Err(user_data_error)
    }

    pub async fn create_pg_database(&self, db_name: &str, user: &str) -> Result<(), DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().create_pg_database(db_name, user).await
    }

    pub async fn create_pg_tables(&self) -> Result<(), DbApiError> {
        check_postgres_connection!(self);

        let create_schema_result =
            self.postgres_connection.as_ref().unwrap().create_pg_schema().await;

        if create_schema_result.is_err() {
            return create_schema_result;
        }

        self.postgres_connection.as_ref().unwrap().create_pg_tables().await
    }

    pub async fn create_pg_user(&self, username: &str, password: &str) -> Result<(), DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().create_pg_user(username, password).await
    }

    pub async fn create_session(&self, user_id: i32, is_lts: bool) -> Result<SessionData, SessionError> {
        check_redis_connection!(self);

        self.redis_connection.as_ref().unwrap().create_session(user_id, is_lts).await
    }

    pub async fn destroy_session(&self, session_id: &str) -> Result<(), SessionError> {
        check_redis_connection!(self);

        self.redis_connection.as_ref().unwrap().destroy_session(session_id).await
    }

    pub async fn drop_pg_db(&self) -> Result<(), DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().drop_pg_db().await
    }

    pub async fn drop_pg_tables(&self) -> Result<(), DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().drop_pg_schema().await
    }

    pub async fn drop_pg_db_user(&self, username: &str) -> Result<(), DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().drop_pg_db_user(username).await
    }

    pub async fn get_session_data(&self, session: &Session, session_id: &str, force_session_renew: bool) -> Result<SessionData, SessionError> {
        check_redis_connection!(self);

        let redis_connection = self.redis_connection.as_ref().unwrap();
        let session_data = redis_connection.get_session_data(session_id).await;

        if session_data.is_ok() {
            let session_data = session_data.as_ref().ok().unwrap();

            redis_connection.renew_session(session, session_data, force_session_renew).await;
        }

        session_data
    }

    pub async fn get_upload_data(&self, upload_id: i32) -> Result<UploadData, DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().get_upload_data(upload_id).await
    }

    pub async fn get_uploads(&self, start_id: i32, max_count: i16, show_sfw: bool, show_nsfw: bool) -> Result<UploadPrvList, DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().get_uploads(start_id, max_count, show_sfw, show_nsfw).await
    }

    pub async fn get_uploads_range(&self, start_id: i32, end_id: i32, show_sfw: bool, show_nsfw: bool) -> Result<UploadPrvList, DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().get_uploads_range(start_id, end_id, show_sfw, show_nsfw).await
    }

    pub async fn get_userdata_by_id(&self, user_id: i32) -> Result<UserData, DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().get_userdata_by_id(user_id).await
    }

    pub async fn get_userdata_by_username(&self, username: &str) -> Result<UserData, DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().get_userdata_by_username(username).await
    }

    pub fn have_postgres_connection(&self) -> bool {
        self.postgres_connection.is_some()
    }

    pub fn have_redis_connection(&self) -> bool {
        self.redis_connection.is_some()
    }

    pub async fn new(project_config: &ProjectConfig, require_postgres: bool, require_redis: bool) -> Result<DbConnection, DbApiError> {
        trace!("Enter DbConnection::new");

        let postgres_connection = PostgresConnection::new(project_config).await;
        let redis_connection = RedisConnection::new(project_config).await;

        if postgres_connection.is_none() && require_postgres {
            return Err(DbApiError::new(ConnectionError, "Keine Verbindung zum Postgres Server vorhanden!"));
        }
        else if redis_connection.is_none() && require_redis {
            return Err(DbApiError::new(ConnectionError, "Keine Verbindung zum Redis Server vorhanden!"));
        }

        let db_connection = DbConnection {
            postgres_connection,
            redis_connection,
        };

        Ok(db_connection)
    }

    pub async fn new_pg_root_connection(project_config: &ProjectConfig, user: &str, password: &str) -> Result<DbConnection, DbApiError> {
        let postgres_connection = PostgresConnection::new_root_connection(project_config, user, password).await;

        if postgres_connection.is_none() {
            return Err(DbApiError::new(ConnectionError, "Keine Verbindung zum Postgres Server vorhanden!"));
        }

        let db_connection = DbConnection {
            postgres_connection,
            redis_connection: None,
        };

        Ok(db_connection)
    }

    // TODO: Implement
    //pub fn search_uploads(&self, search_string: &str, start_id: i32, amount: i16, show_nsfw: bool) -> Result<UploadPrvList, DbApiError> {
    //    trace!("Enter DbConnection::search_uploads");
    //
    //    Err(DbApiError::new(UnknownError, "Unbekannter Fehler"))
    //}

    pub async fn vote_comment(&self, comment_id: i32, user_id: i32, vote_value: i32) -> Result<(), DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().vote_comment(comment_id, user_id, vote_value).await
    }

    pub async fn vote_tag(&self, tum_id: i32, user_id: i32, vote_value: i32) -> Result<(), DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().vote_tag(tum_id, user_id, vote_value).await
    }

    pub async fn vote_upload(&self, upload_id: i32, user_id: i32, vote_value: i32) -> Result<(), DbApiError> {
        check_postgres_connection!(self);

        self.postgres_connection.as_ref().unwrap().vote_upload(upload_id, user_id, vote_value).await
    }
}