use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, middleware};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};
use sailfish::TemplateOnce;
use veruna::models::Page;

use veruna::schema::pages::dsl::pages;

#[derive(TemplateOnce)]
#[template(path = "page.stpl")]
struct PageModel {
    id: i32,
    header: String,
}

fn page_by_id(connection: PooledConnection<ConnectionManager<SqliteConnection>>, id: i32) -> Page {
    let page: Page = pages
        .find(id)
        .first(&connection)
        .expect("Error loading post");
    page
}

async fn index(req: HttpRequest, db: web::Data<Pool<ConnectionManager<SqliteConnection>>>) -> actix_web::Result<HttpResponse> {
    let id_string = req.match_info().get("id").unwrap().to_string();
    let id = id_string.parse::<i32>().unwrap();

    let conn = db.get().expect("couldn't get db connection from pool");

    let page = web::block(move || page_by_id(conn, id))
        .await?;

    let page_model = PageModel{
        id,
        header: page.name
    };

    let body = page_model
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let manager = diesel::r2d2::ConnectionManager::<SqliteConnection>::new("veruna.sqlite");
    let pool = diesel::r2d2::Pool::new(manager).unwrap();

    log::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/diesel-{id}").route(web::get().to(index)),
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
