use std::collections::HashMap;
use actix_web::{Error, HttpResponse};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use entity::site::Model;
use sea_orm::{DatabaseConnection};
use view::models::main_page_view::MainPageView;
use crate::services::component_service::get_node_components;
use crate::services::db_service::{get_table_data, get_table_info};
use sailfish::TemplateOnce;

pub async fn main_page_action(connection: &DatabaseConnection, site: Model, main_page_id: i32)
                              -> actix_web::Result<HttpResponse, Error> {
    let components_result = get_node_components(
        connection,
        main_page_id,
    ).await;

    if let Err(e) = components_result {
        return Ok(HttpResponse::from_error(e));
    }

    let table_info_result = get_table_info(
        connection,
        "article".to_string(),
    ).await;

    if let Err(e) = table_info_result {
        return Ok(HttpResponse::from_error(e));
    }

    let table_info = table_info_result.unwrap();
    let table_data_result = get_table_data(
        connection,
        "article".to_string(),
    ).await;

    if let Err(e) = table_data_result {
        return Ok(HttpResponse::from_error(e));
    }

    let table_data = table_data_result.unwrap();
    let mut components = Vec::new();
    components.push(table_data);

    for component in &components {
        for item in &component.list {
            println!("{}: {}", item.key, item.value)
        }
    }

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