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
use argonautica::{Hasher, Verifier};
use argonautica::config::Variant;
use crate::db_api::db_result::{SessionError, SessionData};
use crate::db_api::DbConnection;
use actix_session::Session;
use crate::security::AccessLevel::User;
use crate::db_api::db_result::SessionErrorType::NoSession;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum AccessLevel {
    None = 0,
    User = 1,
    Mod = 2, // -> Is gay
}

pub fn check_and_escape_comment(comment: &str) -> Option<String> {
    let comment_length = comment.len();

    // A comment can be 8000 characters long
    if comment_length <= 8000 {
        let comment_chars = comment.chars();

        // A comment can only have alphanumeric characters, ascii punctuations (like !;:% etc.) and ascii whitespaces
        for char in comment_chars {
            let char_is_alphanumeric = char.is_alphanumeric();
            let char_is_ascii_punctuation = char.is_ascii_punctuation();
            let char_is_ascii_whitespace = char.is_ascii_whitespace();

            if !char_is_alphanumeric && !char_is_ascii_punctuation && !char_is_ascii_whitespace {
                return None;
            }
        }

        return Some(escape(comment).to_string());
    }
    else {
        return None;
    }
}

pub fn check_filename(filename: &str) -> bool {
    let filename_length = filename.len();

    // A filename can be up to 32 characters long (including file extension) and have to contain exactly one point (for file extension)
    if filename_length <= 32 {
        let filename_chars = filename.chars();

        // A tag can only have ascii alphanumeric characters and simple whitespaces
        for char in filename_chars {
            let char_is_ascii_alphanumeric = char.is_ascii_alphanumeric();
            let char_is_allowed_special_character = char == '-' || char == '_' || char == '.';

            if !char_is_ascii_alphanumeric && !char_is_allowed_special_character {
                return false;
            }
        }

        return true;
    }
    else {
        return false;
    }
}

pub fn check_invite_key(invite_key: &str) -> bool {
    let key_length = invite_key.len();

    // An invite key have to be 32 characters long
    if key_length == 32 {
        let key_chars = invite_key.chars();

        // A invite key can only have ascii alphanumeric characters
        for char in key_chars {
            let char_is_ascii_alphanumeric = char.is_ascii_alphanumeric();

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

pub fn check_password(password: &str) -> bool {
    let password_length = password.len();

    // A password can be 64 characters long
    if password_length <= 64 {
        let password_chars = password.chars();

        // A password can only have alphanumeric characters and ascii punctuations (like !;:% etc.)
        for char in password_chars {
            let char_is_alphanumeric = char.is_alphanumeric();
            let char_is_ascii_punctuation = char.is_ascii_punctuation();

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

pub fn check_tag(tag: &str) -> bool {
    let tag_length = tag.len();

    // A tag can be 64 characters long
    if tag_length <= 64 {
        let tag_chars = tag.chars();

        // A tag can only have ascii alphanumeric characters and simple whitespaces
        for char in tag_chars {
            let char_is_ascii_alphanumeric = char.is_ascii_alphanumeric();
            let char_is_space = char == ' ';

            if !char_is_ascii_alphanumeric && !char_is_space {
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
        for char in username_chars {
            let char_is_ascii_alphanumeric = char.is_ascii_alphanumeric();

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

pub async fn get_user_session(db_connection: &DbConnection, session: &Session, force_session_renew: bool) -> Result<SessionData, SessionError> {
    let session_id = session.get::<String>("session_id");

    if session_id.is_ok() {
        let session_id = session_id.unwrap().unwrap_or("".to_owned());

        if session_id != "" {
            let session_data = db_connection.get_session_data(session, session_id.as_str(), force_session_renew).await;

            if session_data.is_ok() {
                let session_data = session_data.ok().unwrap();

                return Ok(session_data);
            }
            else {
                let session_error = session_data.err().unwrap();
                return Err(session_error);
            }
        }
        else {
            let session_error = SessionError::new(NoSession, "Keine Session ID gespeichert");

            return Err(session_error);
        }
    }
    else {
        let session_error = SessionError::new(NoSession, "Keine Session ID gespeichert");

        return Err(session_error);
    }
}

pub fn hash_password(password: &str, secret_key: &str) -> Option<String> {
    let mut argon2_hasher = Hasher::default();

    // Don't block all CPU cores
    argon2_hasher.configure_lanes(2);
    argon2_hasher.configure_threads(2);

    argon2_hasher.with_password(password);
    argon2_hasher.with_secret_key(secret_key);

    let hash_result = argon2_hasher.hash();

    if hash_result.is_ok() {
        return Some(hash_result.unwrap());
    }
    else {
        return None;
    }
}

pub fn verify_password(password_hash: &str, password: &str, secret_key: &str) -> Option<bool> {
    let mut argon2_verifier = Verifier::default();

    // Don't block all CPU cores
    argon2_verifier.configure_threads(2);

    argon2_verifier.with_hash(password_hash);
    argon2_verifier.with_password(password);
    argon2_verifier.with_secret_key(secret_key);

    let verify_result = argon2_verifier.verify();

    if verify_result.is_ok() {
        return Some(verify_result.unwrap());
    }
    else {
        return None;
    }
}