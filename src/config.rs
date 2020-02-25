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

enum ConnectionMethod {
    Tcp,
    Udp,
    UnixSocket,
}

struct ConfigField<T> {
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

struct Application {
    max_upload_size: ConfigField<u16>,
}

impl Application {
    pub fn new() -> Application {
        Application {
            max_upload_size: ConfigField::new_empty(0),
        }
    }

    pub fn parse_toml(&mut self, toml_obj: &Value) {
        //let toml_obj: Value = toml::from_str(toml_string.as_str())?;

        let toml_max_upload_size = toml_obj["Application"]["max_upload_size"].as_integer();
        
        if toml_max_upload_size.is_some() {
            self.max_upload_size.set_value(toml_max_upload_size.unwrap() as u16);
        }
    }
}

struct FilesystemConfig {
    ffprobe_path: ConfigField<String>,
    userconfig_filepath: ConfigField<String>,
    static_webcontent_path: ConfigField<String>,
    uploads_path: ConfigField<String>,
}

impl FilesystemConfig {
    pub fn new() -> FilesystemConfig {
        FilesystemConfig {
            ffprobe_path: ConfigField::new_empty(String::new()),
            userconfig_filepath: ConfigField::new_empty(String::new()),
            static_webcontent_path: ConfigField::new_empty(String::new()),
            uploads_path: ConfigField::new_empty(String::new()),
        }
    }

    pub fn parse_toml(&mut self, toml_obj: &Value) {
        let ffprobe_path = toml_obj["FilesystemConfig"]["ffprobe_path"].as_str();
        let userconfig_filepath = toml_obj["FilesystemConfig"]["userconfig_filepath"].as_str();
        let static_webcontent_path = toml_obj["FilesystemConfig"]["static_webcontent_path"].as_str();
        let uploads_path = toml_obj["FilesystemConfig"]["uploads_path"].as_str();

        if ffprobe_path.is_some() {
            self.ffprobe_path.set_value(ffprobe_path.unwrap().to_owned());
        }

        if userconfig_filepath.is_some() {
            self.userconfig_filepath.set_value(userconfig_filepath.unwrap().to_owned());
        }

        if static_webcontent_path.is_some() {
            self.static_webcontent_path.set_value(static_webcontent_path.unwrap().to_owned());
        }

        if uploads_path.is_some() {
            self.uploads_path.set_value(uploads_path.unwrap().to_owned());
        }
    }
}

struct NetworkConfig {
    ip_addr: ConfigField<String>,
    port: ConfigField<u16>,
}

struct PostgresConfig {
    host: ConfigField<String>,
    port: ConfigField<u16>,
    unix_socket_dir: ConfigField<String>,
    connection_method: ConfigField<ConnectionMethod>,
    user: ConfigField<String>,
    password: ConfigField<String>,
    db_name: ConfigField<String>,
    required_schema_version: ConfigField<u32>,
}

struct RedisConfig {
    host: ConfigField<String>,
    port: ConfigField<u16>,
    unix_socket_file: ConfigField<String>,
    connection_method: ConfigField<ConnectionMethod>,
}

struct ProjectConfig {
    application_config: Application,
    filesystem_config: FilesystemConfig,
    network_config: NetworkConfig,
    postgres_config: PostgresConfig,
    redis_config: RedisConfig,
}