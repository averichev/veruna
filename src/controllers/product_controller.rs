use actix_web::{Error, HttpResponse, web};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use entity::product;
use entity::product::Entity as ProductEntity;
use view::models::product_view::ProductView;
use sea_orm::{entity::*};
use sailfish::TemplateOnce;
use crate::AppState;

pub async fn product_page(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let product: product::Model = ProductEntity::find_by_id(id.into_inner())
        .one(conn)
        .await
        .expect("could not find post")
        .unwrap();

    let product_view = ProductView {
        id: product.id,
        name: product.name,
    };

    let body = product_view
        .render_once()
        .map_err(|e| InternalError::new(
            e,
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}