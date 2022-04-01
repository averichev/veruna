use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web::error::{InternalError};
use actix_web::http::StatusCode;
use futures_util::TryFutureExt;
use repository::host_repository;
use repository::host_repository::find_by_name;
use crate::AppState;

pub async fn path_test(
    req: HttpRequest,
    data: web::Data<AppState>
)
    -> actix_web::Result<HttpResponse, Error>
{
    let conn = &data.conn;
    let path = req
        .path()
        .to_string();
    let host_info: Vec<String> = req
        .connection_info()
        .host()
        .split(":")
        .map(|s| s.to_string())
        .filter(|v| !v.is_empty())
        .collect();
    let host = host_info.get(0).unwrap().to_string();
    let host_model = find_by_name(&host, conn)
        .await;

    if let Err(e) = host_model {
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("DB error {}", e.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )));
    }
    if host_model.unwrap().is_none() {
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("{} not found", &host.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )));
    }

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
    let response = HttpResponse::MovedPermanently()
        .insert_header(("Location", "/static/favicon.ico"))
        .finish();
    Ok(
        response
    )
}