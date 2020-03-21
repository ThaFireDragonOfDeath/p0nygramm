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

use crate::security::check_tag;
use log::{trace, debug, info, warn, error};

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
    pub keep_logged_in: bool,
}

#[derive(Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub password: String,
    pub invite_key: String,
}

pub struct TagData {
    pub taglist: Vec<String>,
}

impl TagData {
    pub fn from_str(taglist_str: &str) -> TagData {
        let mut result_vec : Vec<String> = Vec::new();
        let tags_iter : Vec<&str> = taglist_str.split(",").collect();

        for tag in tags_iter {
            let mut current_tag = tag.to_owned();

            // Remove all whitespaces from left
            while current_tag.starts_with(" ") {
                current_tag.remove(0);
            }

            // Remove all whitespaces from right
            while current_tag.ends_with(" ") {
                current_tag.remove(current_tag.len() - 1);
            }

            let tag_is_ok = check_tag(current_tag.as_str());

            if tag_is_ok {
                result_vec.push(current_tag);
            }
            else {
                warn!("from_str: Got invalid tag: {}", current_tag);
            }
        }

        TagData {
            taglist: result_vec,
        }
    }
}