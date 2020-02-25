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

extern crate v_htmlescape;
use v_htmlescape::escape;

pub fn check_and_escape_comment(comment: &str) -> Option<String> {
    return None;
}

pub fn check_password(username: &str) -> bool {
    let username_lenght = username.len();

    // A password can be 64 characters long
    if username_lenght <= 64 {
        let username_chars = username.chars();

        // A password can only have alphanumeric characters and ascii punctuations (like !;:% etc.)
        for username_char in username_chars {
            let char_is_alphanumeric = username_char.is_alphanumeric();
            let char_is_ascii_punctuation = username_char.is_ascii_punctuation();

            if !char_is_alphanumeric && !char_is_ascii_punctuation {
                return false;
            }
        }

        return true;
    }
    else {
        return false;
    }
}

pub fn check_username(username: &str) -> bool {
    let username_length = username.len();

    // A username can be 32 characters long
    if username_length <= 32 {
        let username_chars = username.chars();

        // A username can only have ascii alphanumeric characters
        for username_char in username_chars {
            let char_is_ascii_alphanumeric = username_char.is_ascii_alphanumeric();

            if !char_is_ascii_alphanumeric {
                return false;
            }
        }

        return true;
    }
    else {
        return false;
    }
}
