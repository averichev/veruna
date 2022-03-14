mod controllers;

use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, middleware};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::middleware::NormalizePath;
use actix_web::middleware::TrailingSlash::{Always, Trim};
use sailfish::TemplateOnce;
use crate::controllers::product_controller::product_page;
use crate::controllers::path::path_test;
use sea_orm::DatabaseConnection;
use sea_orm::{entity::*, query::*};

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    let state = AppState { conn };
    log::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/product-{id}")
                    .route(web::get().to(product_page)),
            )
            .service(
                web::resource("/{path}*")
                    .route(web::get().to(path_test)),
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
