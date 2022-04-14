use actix_web::{Error, HttpResponse};
use entity::site::Model;
use sea_orm::DatabaseConnection;

pub async fn main_page_action(_connection: &DatabaseConnection, site: Model)
                              -> actix_web::Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("Main page of site {}", site.name)))
}