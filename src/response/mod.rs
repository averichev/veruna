use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;

pub(crate) struct NotFound {
    message: String,
}

impl NotFound {
    pub(crate) fn new(message: String) -> HttpResponse {
        HttpResponse::from_error(InternalError::new(
            message,
            StatusCode::NOT_FOUND,
        ))
    }
}

pub(crate) struct Forbidden {
    message: String,
}

impl Forbidden {
    pub(crate) fn new(message: String) -> HttpResponse {
        HttpResponse::from_error(InternalError::new(
            message,
            StatusCode::FORBIDDEN,
        ))
    }
}

#[derive(Serialize)]
pub(crate) struct LoggedUser{
    pub(crate) username: String
}

#[derive(Serialize)]
pub(crate) struct LoginResponseData{
    pub(crate) user: LoggedUser,
    pub(crate) token: String
}

#[derive(Serialize)]
pub(crate) struct LoginResponse {
    pub(crate) result: bool,
    pub(crate) data: Option<LoginResponseData>
}