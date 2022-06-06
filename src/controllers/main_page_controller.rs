use actix_web::{Error, HttpResponse};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use entity::site::Model;
use sea_orm::{DatabaseConnection};
use view::models::main_page_view::MainPageView;
use crate::services::component_service::get_node_components;
use crate::services::db_service::{get_table_data, get_table_info};
use sailfish::TemplateOnce;
use view::models::component_item::ComponentItem;

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
    let mut components: Vec<ComponentItem> = Vec::new();
    for item in &table_data.list {
        let mut rendered = String::new();
        let mut name = String::new();
        let key = &item.key;
        let value = &item.value;
        if key == "content" {
            rendered.push_str(editorjs::render_to_html(value).as_str());
        }
        if item.key == "name" {
            name.push_str(value.as_str());
        }
        let component_item = ComponentItem {
            html: rendered,
            name: name
        };
        components.push(component_item);
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