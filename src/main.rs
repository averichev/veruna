mod uri;
mod view;

use std::{env, future};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use assert_str::assert_str_eq;
use url::{ParseError, Url};
use veruna_domain::sites::site_kit::SiteKitFactory;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Error, middleware, http};
use actix_web::http::{StatusCode, Uri};
use actix_web::error::InternalError;
use actix_web::web::Data;
use actix_files::Files as Fs;
use actix_web::http::uri::Scheme;
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use dotenv;
use veruna_data::Repositories;
use veruna_domain::sites::{Site, SiteId, SiteReadOption};
use crate::view::MainPageView;
use actix_web::web::Redirect;
use sailfish::TemplateOnce;
use actix_web::http::Uri as ActixUri;

#[derive(Clone)]
pub struct AppState {
    repositories: Arc<dyn veruna_domain::input::Repositories>,
}


async fn test() -> impl Responder{
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("тест")
}

async fn path_test(request: HttpRequest,
                   app: Data<AppState>) -> impl Responder
{
    let repo = app.repositories.site().await;
    let site_kit = SiteKitFactory::build(repo);
    let path = request.path().to_string();
    let tail: String = request.match_info().get("tail").unwrap().parse().unwrap();
    let url = uri::UriParser::parse(request);
    let site = site_kit.get_site(url).await;
    match site {
        None => {
            HttpResponse::from_error(InternalError::new(
                format!("not found site for root node"),
                StatusCode::NOT_FOUND,
            ))
        }
        Some(n) => {
            let view = MainPageView {
                title: format!("{}, {}", path, tail),
                site: view::Site{
                    name: n.0.name().to_string(),
                    description: n.0.description().to_string(),
                }
            };
            let body = view
                .render_once()
                .map_err(|e| InternalError::new(
                    e,
                    StatusCode::INTERNAL_SERVER_ERROR,
                )).unwrap();

            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body)
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let repo = Repositories::new(db_url);
    let state = AppState { repositories: repo };
    log::info!("starting HTTP server at http://localhost:20921");

    let app_factory = move || {
        App::new()
            .wrap(Logger::default())
            .route("{tail:.*}", web::get().to(path_test))
            .app_data(Data::new(state.clone()))
    };

    HttpServer::new(app_factory)
        .bind(("127.0.0.1", 20921))?
        .run()
        .await
}