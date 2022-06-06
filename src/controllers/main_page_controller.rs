use actix_web::{Error, HttpResponse};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use entity::site::Model;
use sea_orm::{DatabaseConnection};
use view::models::main_page_view::MainPageView;
use sailfish::TemplateOnce;
use view::models::component_item::ComponentItem;
use crate::services::action_service::get_components;

pub async fn main_page_action(connection: &DatabaseConnection, main_page_id: i32)
                              -> actix_web::Result<HttpResponse, Error> {

    let components = get_components(connection, main_page_id).await.unwrap();

    let view = MainPageView {
        components
    };

    let body = view
        .render_once()
        .map_err(|e| InternalError::new(
            e,
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?;

    Ok(
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body)
    )
}