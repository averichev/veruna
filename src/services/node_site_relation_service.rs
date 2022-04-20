use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use sea_orm::DbConn;
use entity::node_site::Model as node_site_model;
use repository::site_repository::find_node_site_relation;
use crate::services::internal_db_error;

pub async fn get_node_site_relation(site: entity::site::Model, connection: &DbConn)
                                    -> Result<node_site_model, InternalError<String>>
{
    let node_site_relation_result = find_node_site_relation(
        site,
        &connection,
    )
        .await;

    if let Err(e) = node_site_relation_result {
        return Err(internal_db_error(e));
    }

    let node_site_relation_option = node_site_relation_result.unwrap();

    if node_site_relation_option.is_none() {
        return Err(InternalError::new(
            format!("not found site relation for root node"),
            StatusCode::NOT_FOUND,
        ));
    }

    let node_site_relation_model = node_site_relation_option.unwrap();

    Ok(node_site_relation_model)
}