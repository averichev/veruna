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
pub(crate) struct LoggedUser;

#[derive(Serialize)]
pub(crate) struct LoginResponse {
    pub(crate) result: bool,
    pub(crate) user: Option<LoggedUser>
}