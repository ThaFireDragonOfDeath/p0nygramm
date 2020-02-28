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

use tokio_postgres::{NoTls, Error, Client, Config};
use crate::config::ProjectConfig;
use tokio_postgres::config::SslMode::Disable;
use crate::config::ConnectionMethod::Tcp;
use std::path::Path;
use std::time::Duration;

pub struct PostgresConnection {
    postgres_client: Client,
}

impl PostgresConnection {
    pub async fn new(project_config: &ProjectConfig) -> Option<PostgresConnection> {
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
                    eprintln!("Postgres connection error: {}", connection_error);
                }
            });

            let postgres_connection_object = PostgresConnection {
                postgres_client: client,
            };

            Some(postgres_connection_object)
        }
        else {
            None
        }
    }
}