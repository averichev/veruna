use actix_web::{Error, HttpRequest, HttpResponse};
use actix_web::error::{InternalError};
use actix_web::http::StatusCode;

pub async fn path_test(
    req: HttpRequest
)
    -> actix_web::Result<HttpResponse, Error>
{
    let path = req
        .path()
        .to_string();
    let nodes: Vec<String> = path
        .split("/")
        .map(|s| s.to_string())
        .filter(|v| !v.is_empty())
        .collect();
    let len = nodes.len();
    if len == 0 {
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("Path not found {}", path),
            StatusCode::NOT_FOUND,
        )));
    }
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(path.to_string()))
}


pub async fn redirect_favicon()
    -> actix_web::Result<HttpResponse, Error>
{
    let response =  HttpResponse::MovedPermanently()
        .insert_header(("Location", "/static/favicon.ico"))
        .finish();
    Ok(
       response
    )
}