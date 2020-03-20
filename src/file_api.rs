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

use crate::config::ProjectConfig;

pub struct FileProcessError {
    error_type: FileProcessErrorType,
    error_msg: String,
}

pub enum FileProcessErrorType {
    FormatError,
}

pub fn get_url_from_filename(filename: &str) -> String {
    format!("/uploads/{}", filename)
}

pub fn get_preview_url_from_filename(filename: &str) -> String {
    let filename_point_pos = filename.rfind('.').unwrap();
    let (file_name, _file_ext) = filename.split_at(filename_point_pos);

    format!("/prv/{}.{}", file_name, ".jpg")
}

pub fn process_file(config: ProjectConfig, filename: &str) -> Result<(), FileProcessError> {
    // TODO: Implement

    return Ok(());
}