/*
 * Copyright (C) 2020 Voldracarno Draconor <ThaFireDragonOfDeath@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use redis::{Client, ConnectionInfo, ConnectionAddr, AsyncCommands, RedisResult, Commands, RedisFuture};
use redis::aio::{Connection, MultiplexedConnection};
use crate::config::ProjectConfig;
use crate::config::ConnectionMethod::Tcp;
use std::path::PathBuf;
use crate::db_api::result::{SessionData, DbApiError};
use crate::db_api::result::DbApiErrorType::{UnknownError, NoResult};
use chrono::{ParseResult, DateTime, FixedOffset, Local, Duration};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub struct RedisConnection {
    redis_client: Client,
    redis_connection: MultiplexedConnection,
}

impl RedisConnection {
    pub async fn check_session_exist(&self, session_id: &str) -> Result<bool, DbApiError> {
        Ok(true) // TODO: Implement
    }

    pub async fn create_session(&self, user_id: i32, is_lts: bool) -> Result<SessionData, DbApiError> {
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
                current_iteration += 1;
            }
            else {
                return Err(DbApiError::new(UnknownError, "Unbekannter Fehler"));
            }
        }

        if rand_session_id != "" {
            let redis_key_userid = format!("sessions.{}.user_id", rand_session_id);
            let redis_key_lts = format!("sessions.{}.lts", rand_session_id); // Is long time session (aka keep logged in)

            /*let query_result = redis::pipe().atomic()
                .set(redis_key_userid, user_id).ignore()
                .set(redis_key_lts).ignore()
                .query_async::<MultiplexedConnection, (i32, String, bool)>(&mut redis_connection)
                .await;*/

            // TODO: Finish implementation
        }

        Err(DbApiError::new(UnknownError, "Unbekannter Fehler"))
    }

    pub async fn get_session_data(&self, session_id: &str) -> Result<SessionData, DbApiError> {
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

            if user_id > 0 && session_ttl > 10 {
                let current_time = Local::now();
                let session_expire = current_time + Duration::seconds(session_ttl as i64);
                let session_data = SessionData::new(session_id.to_owned(), user_id, session_expire, is_lts);

                return Ok(session_data);
            }
        }

        return Err(DbApiError::new(NoResult, "Session ID ist ungültig"));
    }

    pub async fn new(project_config: &ProjectConfig) -> Option<RedisConnection> {
        let host = project_config.redis_config.host.get_value();
        let unix_socket_file = project_config.redis_config.unix_socket_file.get_value();
        let port = project_config.redis_config.port.get_value();
        let db_nr = 0;
        let password: Option<String> = None;
        let connection_method = project_config.redis_config.connection_method.get_value();
        let mut connection_addr: ConnectionAddr;

        if connection_method == Tcp {
            connection_addr = ConnectionAddr::Tcp(host, port);
        }
        else {
            connection_addr = ConnectionAddr::Unix(PathBuf::from(unix_socket_file));
        }

        let connection_info = ConnectionInfo {
            addr: Box::from(connection_addr),
            db: db_nr,
            passwd: password,
        };

        let client = redis::Client::open(connection_info);

        if client.is_ok() {
            let client = client.unwrap();
            let mut connection = client.get_multiplexed_tokio_connection().await;

            if connection.is_ok() {
                let mut connection = connection.unwrap();

                let redis_connection_obj = RedisConnection {
                    redis_client: client,
                    redis_connection: connection,
                };

                return Some(redis_connection_obj);
            }
        }

        return None;
    }

    pub async fn renew_session(&self, session_data: &SessionData, force_renew: bool) -> bool {
        let mut redis_connection = self.redis_connection.clone();
        let redis_key_userid = format!("sessions.{}.user_id", session_data.session_id);
        let redis_key_lts = format!("sessions.{}.lts", session_data.session_id); // Is long time session (aka keep logged in)

        // Session duration in hours
        let lts_duration : u32 = 24 * 30; // Long time session are valid for 30 days without activity
        let sts_duration : u32 = 24; // Short time sessions are valid for 1 day without activity

        // Session reload break point
        let lts_break_point : u32 = lts_duration / 2; // If the session have only 15 days remaining, the session will be renewed
        let sts_break_point : u32 = sts_duration / 2; // If the session have only 12 hours remaining, the session will be renewed

        let current_time = Local::now();
        let mut renew_session = false;

        if force_renew {
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
            let new_expire_time = if session_data.is_lts {
                current_time + Duration::hours(lts_duration as i64)
            }
            else {
                current_time + Duration::hours(sts_duration as i64)
            };

            let new_ttl = new_expire_time.signed_duration_since(current_time).num_seconds().abs() as usize;

            let query_result = redis::pipe().atomic()
                .expire(redis_key_userid, new_ttl)
                .expire(redis_key_lts, new_ttl)
                .query_async::<MultiplexedConnection, ((),())>(&mut redis_connection)
                .await;

            if query_result.is_ok() {
                return true;
            }
            else {
                return false;
            }
        }

        return true;
    }
}