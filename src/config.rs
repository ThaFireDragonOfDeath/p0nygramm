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

extern crate toml;

use toml::{Value, de::Error};
use std::convert::TryFrom;
use crate::config::ConnectionMethod::{Tcp, UnixSocket};
use std::path::Path;
use std::fs::File;
use std::io::Read;

#[derive(Copy, Clone)]
pub enum ConnectionMethod {
    Tcp,
    Udp,
    UnixSocket,
}

impl TryFrom<&str> for ConnectionMethod {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value_lowercase = value.to_lowercase();

        if value_lowercase == "tcp" {
            return Ok(Tcp);
        }
        else if value_lowercase == "udp" {
            return Err("UDP is not supported");
        }
        else if value_lowercase == "unixsocket" {
            return Ok(UnixSocket);
        }

        return Err("Failed to parse connection method");
    }
}

pub struct ConfigField<T> {
    value: T,
    is_ro: bool,   //Once a field is set, it can't be changed anymore
}

impl<T: Clone> ConfigField<T> {
    pub fn get_value(&self) -> T {
        return self.value.clone();
    }

    pub fn new_empty(value: T) -> ConfigField<T> {
        ConfigField {
            value,
            is_ro: false,
        }
    }

    pub fn set_value(&mut self, value: T) {
        let is_ro: bool = self.is_ro;

        if !is_ro {
            self.value = value;
            self.is_ro = true;
        }
    }
}

pub struct ApplicationConfig {
    pub max_upload_size: ConfigField<u16>,
}

impl ApplicationConfig {
    pub fn new() -> ApplicationConfig {
        ApplicationConfig {
            max_upload_size: ConfigField::new_empty(0),
        }
    }

    pub fn parse_toml(&mut self, toml_obj: &Value) {
        let toml_max_upload_size = toml_obj["Application"]["max_upload_size"].as_integer();
        
        if toml_max_upload_size.is_some() {
            self.max_upload_size.set_value(toml_max_upload_size.unwrap() as u16);
        }
    }
}

pub struct FilesystemConfig {
    pub ffprobe_path: ConfigField<String>,
    pub default_userconfig_filepath: ConfigField<String>,
    pub static_webcontent_path: ConfigField<String>,
    pub uploads_path: ConfigField<String>,
}

impl FilesystemConfig {
    pub fn new() -> FilesystemConfig {
        FilesystemConfig {
            ffprobe_path: ConfigField::new_empty(String::new()),
            default_userconfig_filepath: ConfigField::new_empty(String::new()),
            static_webcontent_path: ConfigField::new_empty(String::new()),
            uploads_path: ConfigField::new_empty(String::new()),
        }
    }

    pub fn parse_toml(&mut self, toml_obj: &Value) {
        let ffprobe_path = toml_obj["FilesystemConfig"]["ffprobe_path"].as_str();
        let default_userconfig_filepath = toml_obj["FilesystemConfig"]["default_userconfig_filepath"].as_str();
        let static_webcontent_path = toml_obj["FilesystemConfig"]["static_webcontent_path"].as_str();
        let uploads_path = toml_obj["FilesystemConfig"]["uploads_path"].as_str();

        if ffprobe_path.is_some() {
            self.ffprobe_path.set_value(ffprobe_path.unwrap().to_owned());
        }

        if default_userconfig_filepath.is_some() {
            self.default_userconfig_filepath.set_value(default_userconfig_filepath.unwrap().to_owned());
        }

        if static_webcontent_path.is_some() {
            self.static_webcontent_path.set_value(static_webcontent_path.unwrap().to_owned());
        }

        if uploads_path.is_some() {
            self.uploads_path.set_value(uploads_path.unwrap().to_owned());
        }
    }
}

pub struct NetworkConfig {
    pub ip_addr: ConfigField<String>,
    pub port: ConfigField<u16>,
}

impl NetworkConfig {
    pub fn new() -> NetworkConfig {
        NetworkConfig {
            ip_addr: ConfigField::new_empty(String::new()),
            port: ConfigField::new_empty(0),
        }
    }

    pub fn parse_toml(&mut self, toml_obj: &Value) {
        let ip_addr = toml_obj["NetworkConfig"]["ip_addr"].as_str();
        let port = toml_obj["NetworkConfig"]["port"].as_integer();

        if ip_addr.is_some() {
            self.ip_addr.set_value(ip_addr.unwrap().to_owned());
        }

        if port.is_some() {
            self.port.set_value(port.unwrap() as u16);
        }
    }
}

pub struct PostgresConfig {
    pub host: ConfigField<String>,
    pub port: ConfigField<u16>,
    pub unix_socket_dir: ConfigField<String>,
    pub connection_method: ConfigField<ConnectionMethod>,
    pub user: ConfigField<String>,
    pub password: ConfigField<String>,
    pub db_name: ConfigField<String>,
    pub required_schema_version: ConfigField<u32>,
}

impl PostgresConfig {
    pub fn new() -> PostgresConfig {
        PostgresConfig {
            host: ConfigField::new_empty(String::new()),
            port: ConfigField::new_empty(0),
            unix_socket_dir: ConfigField::new_empty(String::new()),
            connection_method: ConfigField::new_empty(Tcp),
            user: ConfigField::new_empty(String::new()),
            password: ConfigField::new_empty(String::new()),
            db_name: ConfigField::new_empty(String::new()),
            required_schema_version: ConfigField::new_empty(0),
        }
    }

    pub fn parse_toml(&mut self, toml_obj: &Value) {
        let host = toml_obj["PostgresConfig"]["host"].as_str();
        let port = toml_obj["PostgresConfig"]["port"].as_integer();
        let unix_socket_dir = toml_obj["PostgresConfig"]["unix_socket_dir"].as_str();
        let connection_method = toml_obj["PostgresConfig"]["connection_method"].as_str();
        let user = toml_obj["PostgresConfig"]["user"].as_str();
        let password = toml_obj["PostgresConfig"]["password"].as_str();
        let db_name = toml_obj["PostgresConfig"]["db_name"].as_str();
        let required_schema_version = toml_obj["PostgresConfig"]["required_schema_version"].as_integer();

        if host.is_some() {
            self.host.set_value(host.unwrap().to_owned());
        }

        if port.is_some() {
            self.port.set_value(port.unwrap() as u16);
        }

        if unix_socket_dir.is_some() {
            self.unix_socket_dir.set_value(unix_socket_dir.unwrap().to_owned());
        }

        if connection_method.is_some() {
            let connection_method_str = connection_method.unwrap();
            let connection_method_obj = ConnectionMethod::try_from(connection_method_str);
            if connection_method_obj.is_ok() {
                self.connection_method.set_value(connection_method_obj.unwrap());
            }
        }

        if user.is_some() {
            self.user.set_value(user.unwrap().to_owned());
        }

        if password.is_some() {
            self.password.set_value(password.unwrap().to_owned());
        }

        if db_name.is_some() {
            self.db_name.set_value(db_name.unwrap().to_owned());
        }

        if required_schema_version.is_some() {
            self.required_schema_version.set_value(required_schema_version.unwrap() as u32);
        }
    }
}

pub struct RedisConfig {
    pub host: ConfigField<String>,
    pub port: ConfigField<u16>,
    pub unix_socket_file: ConfigField<String>,
    pub connection_method: ConfigField<ConnectionMethod>,
}

impl RedisConfig {
    pub fn new() -> RedisConfig {
        RedisConfig {
            host: ConfigField::new_empty(String::new()),
            port: ConfigField::new_empty(0),
            unix_socket_file: ConfigField::new_empty(String::new()),
            connection_method: ConfigField::new_empty(Tcp),
        }
    }

    pub fn parse_toml(&mut self, toml_obj: &Value) {
        let host = toml_obj["RedisConfig"]["host"].as_str();
        let port = toml_obj["RedisConfig"]["port"].as_integer();
        let unix_socket_file = toml_obj["RedisConfig"]["unix_socket_file"].as_str();
        let connection_method = toml_obj["RedisConfig"]["connection_method"].as_str();

        if host.is_some() {
            self.host.set_value(host.unwrap().to_owned());
        }

        if port.is_some() {
            self.port.set_value(port.unwrap() as u16);
        }

        if unix_socket_file.is_some() {
            self.unix_socket_file.set_value(unix_socket_file.unwrap().to_owned());
        }

        if connection_method.is_some() {
            let connection_method_str = connection_method.unwrap();
            let connection_method_obj = ConnectionMethod::try_from(connection_method_str);
            if connection_method_obj.is_ok() {
                self.connection_method.set_value(connection_method_obj.unwrap());
            }
        }
    }
}

pub struct ProjectConfig {
    pub application_config: ApplicationConfig,
    pub filesystem_config: FilesystemConfig,
    pub network_config: NetworkConfig,
    pub postgres_config: PostgresConfig,
    pub redis_config: RedisConfig,
}

impl ProjectConfig {
    pub fn init() -> Option<ProjectConfig> {
        let const_config_str = include_str!("../ressources/config/const-config.toml");
        let system_config_str = include_str!("../ressources/config/system-config.toml");

        let const_config_toml_obj: Result<Value, Error> = toml::from_str(const_config_str);
        let system_config_toml_obj: Result<Value, Error> = toml::from_str(system_config_str);

        if const_config_toml_obj.is_ok() && system_config_toml_obj.is_ok() {
            let mut prj_config = ProjectConfig {
                application_config: ApplicationConfig::new(),
                filesystem_config: FilesystemConfig::new(),
                network_config: NetworkConfig::new(),
                postgres_config: PostgresConfig::new(),
                redis_config: RedisConfig::new(),
            };

            prj_config.parse_toml(&const_config_toml_obj.unwrap());

            let userconfig_path_str = prj_config.filesystem_config.default_userconfig_filepath.get_value();
            let userconfig_path = Path::new(userconfig_path_str.as_str());

            if userconfig_path.is_file() {
                let userconfig_file = File::open(userconfig_path);

                if userconfig_file.is_ok() {
                    let mut userconfig_toml_str = String::new();
                    let read_success = userconfig_file.unwrap().read_to_string(&mut userconfig_toml_str);

                    if read_success.is_ok() {
                        let userconfig_toml_obj: Result<Value, Error> = toml::from_str(userconfig_toml_str.as_str());

                        if userconfig_toml_obj.is_ok() {
                            prj_config.parse_toml(&userconfig_toml_obj.unwrap());
                        }
                    }
                }
            }

            prj_config.parse_toml(&system_config_toml_obj.unwrap());

            Some(prj_config)
        }
        else {
            None
        }
    }

    pub fn parse_toml(&mut self, toml_obj: &Value) {
        self.application_config.parse_toml(toml_obj);
        self.filesystem_config.parse_toml(toml_obj);
        self.network_config.parse_toml(toml_obj);
        self.postgres_config.parse_toml(toml_obj);
        self.redis_config.parse_toml(toml_obj);
    }
}