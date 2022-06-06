use actix_web::error::InternalError;
use sea_orm::DatabaseConnection;
use view::models::component_item::ComponentItem;
use crate::services::component_service::get_node_components;
use crate::services::db_service::{get_table_data, get_table_info};

pub async fn get_components(connection: &DatabaseConnection, main_page_id: i32)
                            -> Result<Vec<ComponentItem>, InternalError<String>>
{
    let components_result = get_node_components(
        connection,
        main_page_id,
    ).await;

    if let Err(e) = components_result {
        return Err(e);
    }

    let table_info_result = get_table_info(
        connection,
        "article".to_string(),
    ).await;

    if let Err(e) = table_info_result {
        return Err(e);
    }

    let table_info = table_info_result.unwrap();
    let table_data_result = get_table_data(
        connection,
        "article".to_string(),
    ).await;

    if let Err(e) = table_data_result {
        return Err(e);
        ;
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
            name: name,
        };
        components.push(component_item);
    }
    return Ok(components);
}