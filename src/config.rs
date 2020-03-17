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
use log::{trace, debug, info, warn, error};

macro_rules! read_toml_entry_connection_method {
    ($self:ident, $toml_obj:ident, $main_entry:expr, $config_name:ident) => {
        let config_section_content = $toml_obj.as_table();

        if config_section_content.is_some() {
            let config_section_content = config_section_content.unwrap().get($main_entry);

            if config_section_content.is_some() {
                let config_section_content = config_section_content.unwrap().as_table();

                if config_section_content.is_some() {
                    let config_value = config_section_content.unwrap().get(stringify!($config_name));

                    if config_value.is_some() {
                        let config_value = config_value.unwrap().as_str();

                        if config_value.is_some() {
                            let connection_method_str = config_value.unwrap();
                            let connection_method_obj = ConnectionMethod::try_from(connection_method_str);

                            if connection_method_obj.is_ok() {
                                $self.$config_name.set_value(connection_method_obj.unwrap());
                            }
                        }
                    }
                }
            }
        }
    };
}

macro_rules! read_toml_entry_number {
    ($self:ident, $toml_obj:ident, $main_entry:expr, $config_name:ident, $config_type:ty) => {
        let config_section_content = $toml_obj.as_table();

        if config_section_content.is_some() {
            let config_section_content = config_section_content.unwrap().get($main_entry);

            if config_section_content.is_some() {
                let config_section_content = config_section_content.unwrap().as_table();

                if config_section_content.is_some() {
                    let config_value = config_section_content.unwrap().get(stringify!($config_name));

                    if config_value.is_some() {
                        let config_value = config_value.unwrap().as_integer();

                        if config_value.is_some() {
                            $self.$config_name.set_value(config_value.unwrap() as $config_type);
                        }
                    }
                }
            }
        }
    };
}

macro_rules! read_toml_entry_string {
    ($self:ident, $toml_obj:ident, $main_entry:expr, $config_name:ident) => {
        let config_section_content = $toml_obj.as_table();

        if config_section_content.is_some() {
            let config_section_content = config_section_content.unwrap().get($main_entry);

            if config_section_content.is_some() {
                let config_section_content = config_section_content.unwrap().as_table();

                if config_section_content.is_some() {
                    let config_value = config_section_content.unwrap().get(stringify!($config_name));

                    if config_value.is_some() {
                        let config_value = config_value.unwrap().as_str();

                        if config_value.is_some() {
                            $self.$config_name.set_value(config_value.unwrap().to_owned());
                        }
                    }
                }
            }
        }
    };
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConnectionMethod {
    Tcp,
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
        read_toml_entry_number!(self, toml_obj, "application", max_upload_size, u16);
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
        read_toml_entry_string!(self, toml_obj, "filesystem", ffprobe_path);
        read_toml_entry_string!(self, toml_obj, "filesystem", default_userconfig_filepath);
        read_toml_entry_string!(self, toml_obj, "filesystem", static_webcontent_path);
        read_toml_entry_string!(self, toml_obj, "filesystem", uploads_path);
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
        read_toml_entry_string!(self, toml_obj, "network", ip_addr);
        read_toml_entry_number!(self, toml_obj, "network", port, u16);
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
        read_toml_entry_string!(self, toml_obj, "postgres", host);
        read_toml_entry_number!(self, toml_obj, "postgres", port, u16);
        read_toml_entry_string!(self, toml_obj, "postgres", unix_socket_dir);
        read_toml_entry_connection_method!(self, toml_obj, "postgres", connection_method);
        read_toml_entry_string!(self, toml_obj, "postgres", user);
        read_toml_entry_string!(self, toml_obj, "postgres", password);
        read_toml_entry_string!(self, toml_obj, "postgres", db_name);
        read_toml_entry_number!(self, toml_obj, "postgres", required_schema_version, u32);
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
        read_toml_entry_string!(self, toml_obj, "redis", host);
        read_toml_entry_number!(self, toml_obj, "redis", port, u16);
        read_toml_entry_string!(self, toml_obj, "redis", unix_socket_file);
        read_toml_entry_connection_method!(self, toml_obj, "redis", connection_method);
    }
}

pub struct SecurityConfig {
    pub password_hash_key: ConfigField<String>,
    pub session_private_key: ConfigField<String>,
    pub register_password: ConfigField<String>,
}

impl SecurityConfig {
    pub fn new() -> SecurityConfig {
        SecurityConfig {
            password_hash_key: ConfigField::new_empty(String::new()),
            session_private_key: ConfigField::new_empty(String::new()),
            register_password: ConfigField::new_empty(String::new()),
        }
    }

    pub fn parse_toml(&mut self, toml_obj: &Value) {
        read_toml_entry_string!(self, toml_obj, "security", password_hash_key);
        read_toml_entry_string!(self, toml_obj, "security", session_private_key);
        read_toml_entry_string!(self, toml_obj, "security", register_password);
    }
}

pub struct ProjectConfig {
    pub application_config: ApplicationConfig,
    pub filesystem_config: FilesystemConfig,
    pub network_config: NetworkConfig,
    pub postgres_config: PostgresConfig,
    pub redis_config: RedisConfig,
    pub security_config: SecurityConfig,
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
                security_config: SecurityConfig::new(),
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
            else {
                error!("ProjectConfig::init: Userconf path is invalid");
            }

            prj_config.parse_toml(&system_config_toml_obj.unwrap());

            return Some(prj_config);
        }
        else {
            error!("ProjectConfig::init: Failed to read const and system config");
            return None;
        }
    }

    pub fn parse_toml(&mut self, toml_obj: &Value) {
        self.application_config.parse_toml(toml_obj);
        self.filesystem_config.parse_toml(toml_obj);
        self.network_config.parse_toml(toml_obj);
        self.postgres_config.parse_toml(toml_obj);
        self.redis_config.parse_toml(toml_obj);
        self.security_config.parse_toml(toml_obj);
    }
}