use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use crate::repository::product::product_by_id;
use sailfish::TemplateOnce;

pub async fn product_page(
    req: HttpRequest,
    db: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
)
    -> actix_web::Result<HttpResponse>
{
    let id_string = req.match_info().get("id").unwrap().to_string();
    let id = id_string.parse::<i32>().unwrap();

    let product = web::block(move || {
        let conn = db.get().expect("couldn't get db connection from pool");
        product_by_id(conn, id)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(product) = product {

        let body = product
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("Not found with id: {}", id));
        Ok(res)
    }
}