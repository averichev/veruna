use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use sea_orm::DbErr;

pub mod node_site_relation_service;
pub mod site_service;
pub mod component_service;
pub mod db_service;

pub fn internal_db_error(error: DbErr) -> InternalError<String> {
    InternalError::new(
        format!("DB error {}", error.to_string()),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}