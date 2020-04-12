use tokio_postgres::{NoTls, Error, Client, Config, Statement};
use tokio_postgres::types::ToSql;
use crate::config::ProjectConfig;
use tokio_postgres::config::SslMode::Disable;
use crate::config::ConnectionMethod::Tcp;
use std::path::Path;
use std::time::Duration;
use crate::db_api::db_result::{UploadPrvList, DbApiError, UploadPreview, UploadData, UserData};
use crate::db_api::db_result::DbApiErrorType::{QueryError, UnknownError, NoResult};
use chrono::{DateTime, Local};
use futures::future;
use std::future::Future;
use log::{trace, debug, info, warn, error};

macro_rules! db_schema_version {
    () => { 1 };
}

macro_rules! get_filepath {
    ($filename:expr) => {
        concat!("../../ressources/sql/Schema-v", db_schema_version!(), "/", $filename)
    };
}

pub struct PostgresConnection {
    postgres_client: Client,
}

impl PostgresConnection {
    pub async fn add_comment(&self, comment_poster: i32, comment_upload: i32, comment_text: &str) -> Result<(), DbApiError> {
        trace!("Enter PostgresConnection::add_comment");

        let sql_cmd = include_str!(get_filepath!("add_comment.sql"));
        let sql_parameters : &[&(dyn ToSql + Sync)] = &[&comment_poster, &comment_upload, &comment_text];
        let result_rows = self.postgres_client.execute(sql_cmd, sql_parameters).await;

        if result_rows.is_ok() {
            return Ok(());
        }

        error!("PostgresConnection::add_comment: Failed to execute sql statement");
        return Err(DbApiError::new(QueryError, "Fehler beim Ausführen der SQL Anweisung"));
    }

    pub async fn add_tag(&self, tag_text: &str, tag_poster: i32, upload_id: i32) -> Result<(), DbApiError> {
        trace!("Enter PostgresConnection::add_tag");

        let sql_cmd_get_tag_id = include_str!(get_filepath!("get_tag_id.sql"));
        let sql_cmd_add_tag_txt = include_str!(get_filepath!("add_tag_txt.sql"));
        let sql_cmd_add_tag = include_str!(get_filepath!("add_tag.sql"));
        let sql_parameters_1 : &[&(dyn ToSql + Sync)] = &[&tag_text]; // Used for get_tag_id and add_tag_txt
        let mut tag_id : Option<i32> = None;

        // Get tag id
        let result_tag_id = self.postgres_client.query(sql_cmd_get_tag_id, sql_parameters_1).await;

        if result_tag_id.is_ok() {
            let result_tag_id = result_tag_id.unwrap();
            let first_row = result_tag_id.get(0);

            if first_row.is_some() {
                let first_row = first_row.unwrap();
                tag_id = Some(first_row.get(0));
            }
        }

        // Try to add tag text
        if tag_id.is_none() {
            let result_tag_id = self.postgres_client.query(sql_cmd_add_tag_txt, sql_parameters_1).await;

            if result_tag_id.is_ok() {
                let result_tag_id = result_tag_id.unwrap();
                let first_row = result_tag_id.get(0);

                if first_row.is_some() {
                    let first_row = first_row.unwrap();
                    tag_id = Some(first_row.get(0));
                }
            }
        }

        if tag_id.is_some() {
            let sql_parameters_2 : &[&(dyn ToSql + Sync)] = &[&tag_poster, &tag_id, &upload_id]; // Used for add_tag
            let insert_result = self.postgres_client.execute(sql_cmd_add_tag, sql_parameters_2).await;

            if insert_result.is_ok() {
                return Ok(());
            }
        }

        return Err(DbApiError::new(QueryError, "Fehler beim Ausführen der SQL Anweisung"));
    }

    // Returns the upload_id of the new inserted upload or error
    pub async fn add_upload(&self, upload_filename: &str, upload_is_nsfw: bool, uploader: i32) -> Result<i32, DbApiError> {
        trace!("Enter PostgresConnection::add_upload");

        let sql_cmd = include_str!(get_filepath!("add_upload.sql"));
        let upload_is_sfw = !upload_is_nsfw;
        let sql_parameters : &[&(dyn ToSql + Sync)] = &[&upload_filename, &upload_is_sfw, &upload_is_nsfw, &uploader];
        let result_rows = self.postgres_client.query(sql_cmd, sql_parameters).await;

        if result_rows.is_ok() {
            let result_rows = result_rows.unwrap();
            let first_row = result_rows.get(0);

            if first_row.is_some() {
                let first_row = first_row.unwrap();
                let upload_id = first_row.get(0);

                return Ok(upload_id);
            }
            else {
                error!("PostgresConnection::add_upload: Got no result from postgres");
            }
        }
        else {
            error!("PostgresConnection::add_upload: Failed to execute sql statement");
        }

        return Err(DbApiError::new(QueryError, "Fehler beim Ausführen der SQL Anweisung"));
    }

    pub async fn add_user(&self, username: &str, pw_hash: &str, user_is_mod: bool) -> Result<i32, DbApiError> {
        trace!("Enter PostgresConnection::add_user");

        let sql_cmd = include_str!(get_filepath!("add_user.sql"));
        let sql_parameters : &[&(dyn ToSql + Sync)] = &[&username, &pw_hash, &user_is_mod];
        let result_rows = self.postgres_client.query(sql_cmd, sql_parameters).await;

        if result_rows.is_ok() {
            let result_rows = result_rows.unwrap();
            let first_row = result_rows.get(0);

            if first_row.is_some() {
                let first_row = first_row.unwrap();
                let user_id = first_row.get(0);

                return Ok(user_id);
            }
        }

        error!("PostgresConnection::add_user: Failed to execute sql statement");
        return Err(DbApiError::new(QueryError, "Fehler beim Ausführen der SQL Anweisung"));
    }

    pub async fn get_upload_data(&self, upload_id: i32) -> Result<UploadData, DbApiError> {
        trace!("Enter PostgresConnection::get_upload_data");

        let sql_cmd_upload_data = include_str!(get_filepath!("get_uploads.sql"));
        let sql_cmd_comment_data = include_str!(get_filepath!("get_comments_for_upload.sql"));
        let sql_cmd_tag_data = include_str!(get_filepath!("get_uploads.sql"));
        let sql_parameters : &[&(dyn ToSql + Sync)] = &[&upload_id];

        let prepared_statement : Result<(Statement, Statement, Statement), Error> = future::try_join3(
            self.postgres_client.prepare(sql_cmd_upload_data),
            self.postgres_client.prepare(sql_cmd_comment_data),
            self.postgres_client.prepare(sql_cmd_tag_data)
        ).await;

        if prepared_statement.is_ok() {
            let (sql_stm_1, sql_stm_2, sql_stm_3) = prepared_statement.unwrap();
            let result_rows_up = self.postgres_client.query(&sql_stm_1, sql_parameters).await;
            let result_rows_cm = self.postgres_client.query(&sql_stm_2, sql_parameters).await;
            let result_rows_ta = self.postgres_client.query(&sql_stm_3, sql_parameters).await;

            if result_rows_up.is_ok() && result_rows_cm.is_ok() && result_rows_ta.is_ok() {
                let result_rows_up = result_rows_up.unwrap();
                let result_rows_cm = result_rows_cm.unwrap();
                let result_rows_ta = result_rows_ta.unwrap();

                if !result_rows_up.is_empty() {
                    let first_result_row = result_rows_up.get(0).unwrap();
                    let upload_filename : String = first_result_row.get(0);
                    let upload_timestamp : DateTime<Local> = first_result_row.get(1);
                    let upload_is_nsfw : bool = first_result_row.get(2);
                    let uploader_id : i32 = first_result_row.get(3);
                    let uploader_username : String = first_result_row.get(4);
                    let upload_upvotes : i32 = first_result_row.get(5);

                    let mut upload_data = UploadData::new(upload_id, upload_is_nsfw, upload_filename.as_str(),
                                    uploader_id, uploader_username.as_str(), upload_timestamp, upload_upvotes);

                    // Process comments
                    for row in result_rows_cm {
                        let comment_timestamp : DateTime<Local> = row.get(0);
                        let comment_text : String = row.get(1);
                        let comment_poster_id : i32 = row.get(2);
                        let comment_poster_username : String = row.get(3);
                        let comment_upvotes : i32 = row.get(4);

                        upload_data.add_comment(comment_timestamp, comment_text.as_str(),
                                                comment_poster_id, comment_poster_username.as_str(),
                                                comment_upvotes);
                    }

                    // Process tags
                    for row in result_rows_ta {
                        let tag_text : String = row.get(0);
                        let tag_upvotes : i32 = row.get(1);

                        upload_data.add_tag(tag_text.as_str(), tag_upvotes);
                    }
                }
                else {
                    error!("PostgresConnection::get_upload_data: Got no result from sql statement");
                }
            }
            else {
                error!("PostgresConnection::get_upload_data: Failed to execute sql statement");
            }
        }
        else {
            error!("PostgresConnection::get_upload_data: Failed to form prepared statement");
        }

        return Err(DbApiError::new(QueryError, "Fehler beim Ausführen der SQL Anweisung"));
    }

    pub async fn get_uploads(&self, start_id: i32, max_count: i16, show_sfw: bool, show_nsfw: bool) -> Result<UploadPrvList, DbApiError> {
        trace!("Enter PostgresConnection::get_uploads");

        let sql_cmd = include_str!(get_filepath!("get_uploads.sql"));
        let sql_parameters : &[&(dyn ToSql + Sync)] = &[&start_id, &max_count, &show_sfw, &show_nsfw];
        let result_rows = self.postgres_client.query(sql_cmd, sql_parameters).await;

        if result_rows.is_ok() {
            let result_rows_vec = result_rows.unwrap();
            let mut return_vec: Vec<UploadPreview> = Vec::new();

            if !result_rows_vec.is_empty() {
                for row in result_rows_vec {
                    let upload_id = row.get(0);
                    let upload_filename = row.get(1);
                    let upload_is_nsfw = row.get(2);
                    let upload_preview = UploadPreview::new(upload_id, upload_is_nsfw, upload_filename);
                    return_vec.push(upload_preview);
                }
            }
            else {
                warn!("PostgresConnection::get_uploads: No uploads found"); // Usually this should only happen if there is nothing uploaded yet

                return Err(DbApiError::new(NoResult, "Keine Uploads vorhanden"));
            }

            return Ok(UploadPrvList{ uploads: return_vec });
        }
        else {
            error!("PostgresConnection::get_uploads: Failed to execute sql statement");
        }

        return Err(DbApiError::new(QueryError, "Fehler beim Ausführen der SQL Anweisung"));
    }

    pub async fn get_uploads_range(&self, start_id: i32, end_id: i32, show_sfw: bool, show_nsfw: bool) -> Result<UploadPrvList, DbApiError> {
        trace!("Enter PostgresConnection::get_uploads_range");

        let sql_cmd = include_str!(get_filepath!("get_uploads_range.sql"));
        let sql_parameters : &[&(dyn ToSql + Sync)] = &[&start_id, &end_id, &show_sfw, &show_nsfw];
        let result_rows = self.postgres_client.query(sql_cmd, sql_parameters).await;

        if result_rows.is_ok() {
            let result_rows_vec = result_rows.unwrap();
            let mut return_vec: Vec<UploadPreview> = Vec::new();

            if !result_rows_vec.is_empty() {
                for row in result_rows_vec {
                    let upload_id = row.get(0);
                    let upload_filename = row.get(1);
                    let upload_is_nsfw = row.get(2);
                    let upload_preview = UploadPreview::new(upload_id, upload_is_nsfw, upload_filename);
                    return_vec.push(upload_preview);
                }
            }
            else {
                warn!("PostgresConnection::get_uploads: No uploads found"); // Usually this should only happen if there is nothing uploaded yet

                return Err(DbApiError::new(NoResult, "Keine Uploads vorhanden"));
            }

            return Ok(UploadPrvList{ uploads: return_vec });
        }
        else {
            error!("PostgresConnection::get_uploads: Failed to execute sql statement");
        }

        return Err(DbApiError::new(QueryError, "Fehler beim Ausführen der SQL Anweisung"));
    }

    pub async fn get_userdata_by_username(&self, username: &str) -> Result<UserData, DbApiError> {
        trace!("Enter PostgresConnection::get_userdata_by_username");

        let sql_cmd = include_str!(get_filepath!("get_userdata_by_username.sql"));
        let sql_parameters : &[&(dyn ToSql + Sync)] = &[&username];
        let result_rows = self.postgres_client.query(sql_cmd, sql_parameters).await;

        if result_rows.is_ok() {
            let result_rows_vec = result_rows.unwrap();

            if !result_rows_vec.is_empty() {
                let row = result_rows_vec.get(0).unwrap();
                let user_id = row.get(0);
                let user_name = row.get(1);
                let user_pass = row.get(2);
                let user_is_mod = row.get(3);
                let user_data = UserData::new(user_id, user_name, user_pass, user_is_mod);

                return Ok(user_data);
            }
            else {
                warn!("PostgresConnection::get_userdata_by_username: User not found");
            }

            return Err(DbApiError::new(NoResult, "Benutzer ist nicht vorhanden"));
        }
        else {
            error!("PostgresConnection::get_userdata_by_username: Failed to execute sql statement");
        }

        return Err(DbApiError::new(QueryError, "Fehler beim Ausführen der SQL Anweisung"));
    }

    pub async fn new(project_config: &ProjectConfig) -> Option<PostgresConnection> {
        trace!("Enter PostgresConnection::new");

        let host = project_config.postgres_config.host.get_value();
        let unix_socket_dir = project_config.postgres_config.unix_socket_dir.get_value();
        let port = project_config.postgres_config.port.get_value();
        let user = project_config.postgres_config.user.get_value();
        let password = project_config.postgres_config.password.get_value();
        let db_name = project_config.postgres_config.db_name.get_value();
        let connection_method = project_config.postgres_config.connection_method.get_value();
        let mut connection_config = Config::new();

        connection_config.user(user.as_str());
        connection_config.password(password.as_str());
        connection_config.dbname(db_name.as_str());
        connection_config.ssl_mode(Disable);
        connection_config.connect_timeout(Duration::new(2, 0));

        if connection_method == Tcp {
            connection_config.host(host.as_str());
            connection_config.port(port);
        }
        else {
            connection_config.host_path(Path::new(unix_socket_dir.as_str()));
        }

        let connection_result = connection_config.connect(NoTls).await;

        if connection_result.is_ok() {
            let (client, connection) = connection_result.unwrap();

            // The connection object performs the actual communication with the database,
            // so spawn it off to run on its own.
            tokio::spawn(async move {
                let active_connection = connection.await;

                if active_connection.is_err() {
                    let connection_error = active_connection.unwrap_err();
                    error!("PostgresConnection::new: Postgres connection error: {}", connection_error);
                }
            });

            let postgres_connection_object = PostgresConnection {
                postgres_client: client,
            };

            return Some(postgres_connection_object);
        }
        else {
            error!("PostgresConnection::new: Failed to connect to postgres");
        }

        return None;
    }

    pub async fn vote_comment(&self, comment_id: i32, user_id: i32, vote_value: i32) -> Result<(), DbApiError> {
        trace!("Enter PostgresConnection::vote_upload");

        let sql_cmd = include_str!(get_filepath!("vote_comment.sql"));
        let sql_parameters : &[&(dyn ToSql + Sync)] = &[&comment_id, &user_id, &vote_value];
        let result_rows = self.postgres_client.query(sql_cmd, sql_parameters).await;

        if result_rows.is_ok() {
            return Ok(());
        }
        else {
            return Err(DbApiError::new(QueryError, "Fehler beim Ausführen der SQL Anweisung"));
        }
    }

    pub async fn vote_upload(&self, upload_id: i32, user_id: i32, vote_value: i32) -> Result<(), DbApiError> {
        trace!("Enter PostgresConnection::vote_upload");

        let sql_cmd = include_str!(get_filepath!("vote_upload.sql"));
        let sql_parameters : &[&(dyn ToSql + Sync)] = &[&upload_id, &user_id, &vote_value];
        let result_rows = self.postgres_client.query(sql_cmd, sql_parameters).await;

        if result_rows.is_ok() {
            return Ok(());
        }
        else {
            return Err(DbApiError::new(QueryError, "Fehler beim Ausführen der SQL Anweisung"));
        }
    }
}