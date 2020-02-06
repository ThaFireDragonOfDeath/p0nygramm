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

enum ConnectionMethod {
    Tcp,
    Udp,
    UnixSocket,
}

struct ConfigField<T> {
    value: T,
    is_set: bool,   //Once a field is set, it can't be changed anymore
}

struct FilesystemConfig {
    userconfig_filepath: ConfigField<String>,
    static_webcontent_path: ConfigField<String>,
    template_path: ConfigField<String>,
    uploads_path: ConfigField<String>,
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
    filesystem_config: FilesystemConfig,
    network_config: NetworkConfig,
    postgres_config: PostgresConfig,
    redis_config: RedisConfig,
}