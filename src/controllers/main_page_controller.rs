use actix_web::{Error, HttpRequest, HttpResponse, web};
use entity::site::Model;
use sea_orm::DatabaseConnection;

pub async fn main_page_action(connection: &DatabaseConnection, site: Model)
                              -> actix_web::Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("Main page of site {}", site.name)))
}