use crate::security::check_tag;
use log::{trace, debug, info, warn, error};
use mime::Mime;

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
    pub full_success: bool,
}

impl TagData {
    pub fn from_str(taglist_str: &str) -> TagData {
        let mut result_vec : Vec<String> = Vec::new();
        let mut full_success = true;
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
                full_success = false;
                warn!("from_str: Got invalid tag: {}", current_tag);
            }
        }

        TagData {
            taglist: result_vec,
            full_success
        }
    }

    pub fn as_str_ref_vec(&self) -> Vec<&str> {
        let taglist_str_vec = self.taglist.iter().map(|tag| tag.as_str()).collect();

        return taglist_str_vec;
    }
}

pub fn check_file_mime(mime_type: &Mime) -> bool {
    let type_name = mime_type.type_();
    let subtype_name = mime_type.subtype();

    match (type_name, subtype_name) {
        (mime::IMAGE, mime::PNG) => true,
        (mime::IMAGE, mime::JPEG) => true,
        (mime::IMAGE, mime::GIF) => true,
        (mime::VIDEO, mime::MP4) => true,
        _ => false,
    }
}

pub fn check_form_content_mime(mime_type: &Mime) -> bool {
    let type_name = mime_type.type_();
    let subtype_name = mime_type.subtype();

    match (type_name, subtype_name) {
        (mime::APPLICATION, mime::OCTET_STREAM) => true,
        _ => false,
    }
}
