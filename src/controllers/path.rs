use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use sailfish::TemplateOnce;

pub async fn path_test(
    req: HttpRequest
)
    -> actix_web::Result<HttpResponse>
{
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(req.path().to_string()))
}