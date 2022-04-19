use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use sea_orm::DbConn;
use repository::component_node_repository::find_node_components;
use repository::component_repository::find_components_by_id;

pub async fn get_node_components(connection: &DbConn, node_id: i32)
                      -> Result<Vec<entity::component::Model>, InternalError<String>> {
    let node_components_result = find_node_components(
        connection,
        node_id
    ).await;

    if let Err(e) = node_components_result {
        return Err(InternalError::new(
            format!("DB error {}", e.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    let node_components: Vec<entity::component_node::Model> = node_components_result.unwrap();
    let mut nodes_id: Vec<i32> = Vec::new();
    for node_component in node_components {
        nodes_id.push(node_component.component_id)
    }
    let components_result = find_components_by_id(connection, nodes_id).await;
    if let Err(e) = components_result {
        return Err(InternalError::new(
            format!("DB error {}", e.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    let components = components_result.unwrap();
    Ok(components)
}