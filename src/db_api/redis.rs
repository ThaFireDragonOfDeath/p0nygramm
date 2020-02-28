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

use redis::{Client, ConnectionInfo, ConnectionAddr};
use redis::aio::Connection;
use crate::config::ProjectConfig;
use crate::config::ConnectionMethod::Tcp;
use std::path::PathBuf;

pub struct RedisConnection {
    redis_client: Client,
    redis_connection: Connection,
}

impl RedisConnection {
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
            let mut connection = client.get_async_connection().await;

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
}