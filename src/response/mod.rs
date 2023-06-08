use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

pub(crate) struct NotFound{
    message: String
}

impl NotFound{
    pub(crate) fn new(message: String) -> HttpResponse{
        HttpResponse::from_error(InternalError::new(
            message,
            StatusCode::NOT_FOUND,
        ))
    }
}