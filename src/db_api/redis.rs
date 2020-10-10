use redis::{Client, ConnectionInfo, ConnectionAddr, RedisResult, ErrorKind};
use redis::aio::{MultiplexedConnection};
use crate::config::ProjectConfig;
use crate::config::ConnectionMethod::Tcp;
use std::path::PathBuf;
use crate::db_api::db_result::{SessionData, SessionError, SessionErrorType};
use chrono::{Local, Duration};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::db_api::db_result::SessionErrorType::{DbError, SessionInvalid};
use log::{trace, debug, info, warn, error};
use actix_session::Session;

const LTS_DURATION : u32 = 24 * 30; // Long time session are valid for 30 days without activity
const TTL_BUFFER: u8 = 30; // If the value ttl is smaller than this value, the session counts as expired
const STS_DURATION : u32 = 24; // Short time sessions are valid for 1 day without activity

pub struct RedisConnection {
    redis_client: Client,
    redis_connection: MultiplexedConnection,
}

impl RedisConnection {
    pub async fn check_session_exist(&self, session_id: &str) -> Result<bool, SessionError> {
        trace!("Enter RedisConnection::check_session_exist");

        let mut redis_connection = self.redis_connection.clone();
        let redis_key_userid = format!("sessions.{}.user_id", session_id);

        let query_result : RedisResult<(i32, i32)> = redis::pipe().atomic()
            .get(redis_key_userid.as_str())
            .ttl(redis_key_userid.as_str())
            .query_async::<MultiplexedConnection, (i32, i32)>(&mut redis_connection)
            .await;

        if query_result.is_ok() {
            let (user_id, session_ttl) = query_result.unwrap();

            if user_id > 0 && session_ttl > TTL_BUFFER as i32 {
                return Ok(true);
            }
        }
        else {
            let error_kind = query_result.err().unwrap().kind();

            if error_kind == ErrorKind::TypeError {
                debug!("RedisConnection::check_session_exist: Got nil");

                return Ok(false);
            }
            else {
                error!("RedisConnection::check_session_exist: Failed to execute redis command");
                return Err(SessionError::new(DbError, "Fehler beim Zugriff auf die Redis Datenbank"));
            }
        }

        return Err(SessionError::new(SessionErrorType::UnknownError, "Unbekannter Fehler"));
    }

    pub async fn create_session(&self, user_id: i32, is_lts: bool) -> Result<SessionData, SessionError> {
        trace!("Enter RedisConnection::create_session");

        let mut redis_connection = self.redis_connection.clone();
        let mut session_exist = true;
        let mut rand_session_id: String = String::new();
        let max_try : u8 = 5;
        let mut current_iteration : u8 = 0;

        while session_exist {
            rand_session_id = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .collect();

            session_exist = self.check_session_exist(rand_session_id.as_str()).await.unwrap_or(true);

            if current_iteration < max_try {
                warn!("RedisConnection::create_session: Got session id collision");

                current_iteration += 1;
            }
            else {
                error!("RedisConnection::create_session: Got to many session id collisions");

                return Err(SessionError::new(SessionErrorType::UnknownError, "Unbekannter Fehler"));
            }
        }

        if rand_session_id != "" {
            let redis_key_userid = format!("sessions.{}.user_id", rand_session_id);
            let redis_key_lts = format!("sessions.{}.lts", rand_session_id); // Is long time session (aka keep logged in)
            let current_time = Local::now();

            let expire_time = if is_lts {
                current_time + Duration::hours(LTS_DURATION as i64)
            }
            else {
                current_time + Duration::hours(STS_DURATION as i64)
            };

            let new_ttl = (expire_time.signed_duration_since(current_time).num_seconds().abs() as usize) + TTL_BUFFER as usize;

            let query_result = redis::pipe().atomic()
                .set_ex(redis_key_userid, user_id, new_ttl)
                .set_ex(redis_key_lts, is_lts, new_ttl)
                .query_async::<MultiplexedConnection, ((), ())>(&mut redis_connection)
                .await;

            if query_result.is_ok() {
                let session_data = SessionData::new(rand_session_id, user_id, expire_time, is_lts);
                return Ok(session_data);
            }
            else {
                error!("RedisConnection::create_session: Failed to create redis entries");

                return Err(SessionError::new(DbError, "Erstellen der Redis Einträge fehlgeschlagen"));
            }
        }

        error!("RedisConnection::create_session: Unknown error in randomly created session id");

        return Err(SessionError::new(DbError, "Erstellen der Redis Einträge fehlgeschlagen"));
    }

    pub async fn destroy_session(&self, session_id: &str) -> Result<(), SessionError> {
        trace!("Enter RedisConnection::destroy_session");

        let mut redis_connection = self.redis_connection.clone();
        let redis_key_userid = format!("sessions.{}.user_id", session_id);
        let redis_key_lts = format!("sessions.{}.lts", session_id); // Is long time session (aka keep logged in)

        let query_result = redis::pipe().atomic()
            .del(redis_key_userid)
            .del(redis_key_lts)
            .query_async::<MultiplexedConnection, (i32, i32)>(&mut redis_connection)
            .await;

        if query_result.is_ok() {
            return Ok(());
        }

        return Err(SessionError::new(DbError, "Fehler beim Zugriff auf die Redis Datenbank"));
    }

    pub async fn get_session_data(&self, session_id: &str) -> Result<SessionData, SessionError> {
        trace!("Enter RedisConnection::get_session_data");

        let mut redis_connection = self.redis_connection.clone();
        let redis_key_userid = format!("sessions.{}.user_id", session_id);
        let redis_key_lts = format!("sessions.{}.lts", session_id); // Is long time session (aka keep logged in)

        let query_result = redis::pipe().atomic()
            .get(redis_key_userid.as_str())
            .ttl(redis_key_userid.as_str())
            .get(redis_key_lts.as_str())
            .query_async::<MultiplexedConnection, (i32, i32, bool)>(&mut redis_connection)
            .await;

        if query_result.is_ok() {
            let (user_id, session_ttl, is_lts) : (i32, i32, bool) = query_result.unwrap();

            if user_id > 0 && session_ttl > TTL_BUFFER as i32 {
                let current_time = Local::now();
                let session_expire = current_time + Duration::seconds(session_ttl as i64);
                let session_data = SessionData::new(session_id.to_owned(), user_id, session_expire, is_lts);

                return Ok(session_data);
            }
            else {
                info!("RedisConnection::get_session_data: Unknown or invalid session id");

                return Err(SessionError::new(SessionInvalid, "Session ID ist ungültig"));
            }
        }
        else {
            let error_kind = query_result.err().unwrap().kind();

            if error_kind == ErrorKind::TypeError {
                info!("RedisConnection::get_session_data: Unknown or invalid session id");

                return Err(SessionError::new(SessionInvalid, "Session ID ist ungültig"));
            }
            else {
                error!("RedisConnection::get_session_data: Failed to execute Redis query");

                return Err(SessionError::new(DbError, "Fehler beim Zugriff auf die Redis Datenbank"));
            }
        }
    }

    pub async fn new(project_config: &ProjectConfig) -> Option<RedisConnection> {
        trace!("Enter RedisConnection::new");

        let host = project_config.redis_config.host.get_value();
        let unix_socket_file = project_config.redis_config.unix_socket_file.get_value();
        let port = project_config.redis_config.port.get_value();
        let db_nr = 0;
        let username: Option<String> = None;
        let password: Option<String> = None;
        let connection_method = project_config.redis_config.connection_method.get_value();
        let connection_addr: ConnectionAddr;

        if connection_method == Tcp {
            connection_addr = ConnectionAddr::Tcp(host, port);
        }
        else {
            connection_addr = ConnectionAddr::Unix(PathBuf::from(unix_socket_file));
        }

        let connection_info = ConnectionInfo {
            addr: Box::from(connection_addr),
            db: db_nr,
            username,
            passwd: password,
        };

        let client = redis::Client::open(connection_info);

        if client.is_ok() {
            let client = client.unwrap();
            let connection = client.get_multiplexed_tokio_connection().await;

            if connection.is_ok() {
                let connection = connection.unwrap();

                let redis_connection_obj = RedisConnection {
                    redis_client: client,
                    redis_connection: connection,
                };

                return Some(redis_connection_obj);
            }
            else {
                error!("RedisConnection::new: Failed to establish redis connection");
            }
        }

        error!("RedisConnection::new: Failed to construct redis client");

        return None;
    }

    pub async fn renew_session(&self, session: &Session, session_data: &SessionData, force_session_renew: bool) -> bool {
        trace!("Enter RedisConnection::renew_session");

        let mut redis_connection = self.redis_connection.clone();
        let redis_key_userid = format!("sessions.{}.user_id", session_data.session_id);
        let redis_key_lts = format!("sessions.{}.lts", session_data.session_id); // Is long time session (aka keep logged in)

        // Session reload break point
        let lts_break_point : u32 = LTS_DURATION / 2; // If the session have only 15 days remaining, the session will be renewed
        let sts_break_point : u32 = STS_DURATION / 2; // If the session have only 12 hours remaining, the session will be renewed

        let current_time = Local::now();
        let mut renew_session = false;

        if force_session_renew {
            renew_session = true;
        }
        else {
            let expire_time = session_data.expire_datetime;

            let future_time = if session_data.is_lts {
                current_time + Duration::hours(lts_break_point as i64)
            }
            else {
                current_time + Duration::hours(sts_break_point as i64)
            };

            if expire_time < future_time {
                renew_session = true
            }
        }

        if renew_session {
            debug!("RedisConnection::renew_session: Session needs to be renewed");

            let new_expire_time = if session_data.is_lts {
                current_time + Duration::hours(LTS_DURATION as i64)
            }
            else {
                current_time + Duration::hours(STS_DURATION as i64)
            };

            let new_ttl = new_expire_time.signed_duration_since(current_time).num_seconds().abs() as usize;

            let query_result = redis::pipe().atomic()
                .expire(redis_key_userid, new_ttl)
                .expire(redis_key_lts, new_ttl)
                .query_async::<MultiplexedConnection, ((),())>(&mut redis_connection)
                .await;

            if query_result.is_ok() {
                session.renew();

                return true;
            }
            else {
                error!("RedisConnection::renew_session: Failed to execute Redis command");

                return false;
            }
        }
        else {
            debug!("RedisConnection::renew_session: Session don't needs to be renewed");
        }

        return true;
    }
}