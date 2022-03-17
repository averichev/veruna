mod controllers;

use std::env;
use actix_web::{web, App, HttpServer, middleware, guard};
use actix_files::Files as Fs;
use actix_web::web::Data;
use crate::controllers::product_controller::product_page;
use crate::controllers::path::{path_test, redirect_favicon};
use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    let state = AppState { conn };
    log::info!("starting HTTP server at http://localhost:20921");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::default())
            .service(
                web::resource("/favicon.ico")
                    .route(web::route()
                        .guard(guard::Any(guard::Get()))
                        .to(redirect_favicon)
                    )
            )
            .service(Fs::new("/static/", "./static/"))
            .service(
                web::resource("/product-{id}")
                    .route(web::get().to(product_page)),
            )
            .service(
                web::resource("/{path}*")
                    .route(web::get().to(path_test)),
            )
    })
        .bind(("127.0.0.1", 20921))?
        .run()
        .await
}
