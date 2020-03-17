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

use crate::db_api::db_result;

#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum ErrorCode {
    DatabaseError,
    UserInputError,
    NoResult,
    Unauthorized,
    Ignored,
}

#[derive(Serialize, Deserialize)]
pub struct BackendError {
    error_code: ErrorCode,
    error_msg: String,
}

impl BackendError {
    pub fn new(error_code: ErrorCode, error_msg: &str) -> BackendError {
        BackendError {
            error_code,
            error_msg: error_msg.to_owned(),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct UserData {
    pub user_id: i32,
    pub username: String,
    pub user_is_mod: bool,
}

impl UserData {
    pub fn new(db_userdata: &db_result::UserData) -> UserData {
        UserData {
            user_id: db_userdata.user_id,
            username: db_userdata.username.clone(),
            user_is_mod: db_userdata.user_is_mod,
        }
    }
}