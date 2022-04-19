use actix_web::{Error, HttpResponse};
use entity::site::Model;
use sea_orm::DatabaseConnection;
use crate::services::component_service::get_node_components;

pub async fn main_page_action(connection: &DatabaseConnection, site: Model, main_page_id: i32)
                              -> actix_web::Result<HttpResponse, Error> {
    let components_result = get_node_components(
        connection,
        main_page_id
    ).await;

    if let Err(e) = components_result{
        return Ok(HttpResponse::from_error(e));
    }

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("Main page of site {}, {}", site.name, main_page_id)))
}