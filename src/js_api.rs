use actix_web::{HttpResponse, web};
use crate::config::ProjectConfig;
use actix_session::Session;
use crate::backend_api::request_data::{CommentData, RegisterData, LoginData};
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::StatusCode;
use actix_multipart::Multipart;
use crate::backend_api::response_result::{Filter, BackendError, UserExists, AddUploadSuccess, UserData, SuccessReport};
use crate::db_api::db_result::{UploadPrvList, UploadData};

macro_rules! implement_jsapi_func_backend_call {
    ($func_name:ident, $( $func_arg_names:ident: $func_arg_types:ty ),+) => {
        crate::backend_api::$func_name( $( &$func_arg_names ),+ ).await;
    };
}

macro_rules! implement_jsapi_func_body {
    ($response_obj:ident) => {
        if $response_obj.is_ok() {
            let response_obj = $response_obj.ok().unwrap();
            let response_txt = serde_json::to_string(&response_obj).unwrap_or("".to_owned());

            return HttpResponse::Ok().body(response_txt);
        }
        else {
            let response_obj = $response_obj.err().unwrap();
            let response_txt = serde_json::to_string(&response_obj).unwrap_or("".to_owned());
            let status_code = StatusCode::from_u16(response_obj.http_status_code).unwrap();
            let http_response = HttpResponseBuilder::new(status_code).body(response_txt);

            return http_response;
        }
    };
}

macro_rules! implement_jsapi_func {
    ($func_name:ident, $( $func_arg_names:ident: $func_arg_types:ty ),+) => {
        pub async fn $func_name( $( $func_arg_names: $func_arg_types ),+ ) -> HttpResponse {
            let response_obj = implement_jsapi_func_backend_call!($func_name, $( $func_arg_names: $func_arg_types ),+);

            implement_jsapi_func_body!(response_obj);
        }
    };
}

implement_jsapi_func!(add_comment, config: web::Data<ProjectConfig>, session: Session, comment_data: web::Form<CommentData>);

// Because of the mut payload some parts of this method have to be written by hoof
pub async fn add_upload(config: web::Data<ProjectConfig>, session: Session, mut payload: Multipart) -> HttpResponse {
    let response_obj = crate::backend_api::add_upload(&config, &session, &mut payload).await;

    implement_jsapi_func_body!(response_obj);
}

implement_jsapi_func!(check_username_exists, config: web::Data<ProjectConfig>, url_data: web::Path<String>);

implement_jsapi_func!(get_filter, config: web::Data<ProjectConfig>, session: Session);

implement_jsapi_func!(get_uploads, config: web::Data<ProjectConfig>, session: Session, url_data: web::Path<(i32, i16, bool, bool)>);

implement_jsapi_func!(get_uploads_range, config: web::Data<ProjectConfig>, session: Session, url_data: web::Path<(i32, i32, bool, bool)>);

implement_jsapi_func!(get_upload_data, config: web::Data<ProjectConfig>, session: Session, url_data: web::Path<i32>);

implement_jsapi_func!(get_userdata_by_username, config: web::Data<ProjectConfig>, session: Session, url_data: web::Path<String>);

implement_jsapi_func!(login, config: web::Data<ProjectConfig>, session: Session, login_data: web::Form<LoginData>);

implement_jsapi_func!(logout, config: web::Data<ProjectConfig>, session: Session);

implement_jsapi_func!(register, config: web::Data<ProjectConfig>, register_data: web::Form<RegisterData>);

implement_jsapi_func!(set_filter, config: web::Data<ProjectConfig>, session: Session, url_data: web::Path<(bool, bool)>);

implement_jsapi_func!(vote_comment, config: web::Data<ProjectConfig>, session: Session, url_data: web::Path<(i32, i32)>);

implement_jsapi_func!(vote_tag, config: web::Data<ProjectConfig>, session: Session, url_data: web::Path<(i32, i32)>);

implement_jsapi_func!(vote_upload, config: web::Data<ProjectConfig>, session: Session, url_data: web::Path<(i32, i32)>);