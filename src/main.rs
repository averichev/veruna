use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, middleware};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use r2d2_sqlite::SqliteConnectionManager;
use sailfish::TemplateOnce;

mod db;

use db::{Pool, Queries};
use db::Page as PageEntity;

#[derive(TemplateOnce)]
#[template(path = "page.stpl")]
struct Page<'a> {
    id: &'a i32,
    header: String
}

async fn page(req: HttpRequest, db: web::Data<Pool>) -> actix_web::Result<HttpResponse> {
    let id_string = req.match_info().get("id").unwrap().to_string();
    let id = &id_string.parse::<i32>().unwrap();

    let pageResult = db::execute(&db, Queries::GetById).await?;
    let page: PageEntity  = pageResult.get(0).unwrap().clone();

    // let header  = "wdfsdf".to_string();
    // let header  = page.header;
    let page_model = Page{
        id,
        header: page.header
    };

    let body = page_model
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // connect to SQLite DB
    let manager = SqliteConnectionManager::file("veruna.sqlite");
    let pool = Pool::new(manager).unwrap();

    log::info!("starting HTTP server at http://localhost:8080");
    // start HTTP server
    HttpServer::new(move || {
        App::new()
            // store db pool as Data object
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/page-{id}").route(web::get().to(page)),
            )
    })
        .bind(("127.0.0.1", 8080))?
        .workers(2)
        .run()
        .await
}
