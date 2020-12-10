use v_htmlescape::escape;
use crate::db_api::db_result::{SessionError, SessionData};
use crate::db_api::DbConnection;
use actix_session::Session;
use crate::db_api::db_result::SessionErrorType::NoSession;
use argon2::{Config, ThreadMode, Variant, Version};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

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
    if password_length >= 8 && password_length <= 64 {
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
    let salt: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect();

    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 4096,
        time_cost: 192,
        lanes: 2,
        thread_mode: ThreadMode::Parallel,
        secret: secret_key.as_ref(),
        ad: &[],
        hash_length: 32
    };

    let hash_result = argon2::hash_encoded(password.as_ref(), salt.as_ref(), &config);

    if hash_result.is_ok() {
        return Some(hash_result.unwrap());
    }
    else {
        return None;
    }
}

pub fn verify_password(password_hash: &str, password: &str, secret_key: &str) -> Option<bool> {
    let verify_result = argon2::verify_encoded_ext(password_hash, password.as_ref(), secret_key.as_ref(), &[]);

    if verify_result.is_ok() {
        return Some(verify_result.unwrap());
    }
    else {
        return None;
    }
}