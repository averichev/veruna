mod uri;
mod view;
mod response;

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
use crate::view::{MainPageView, NodeView};
use actix_web::web::Redirect;
use sailfish::TemplateOnce;
use actix_web::http::Uri as ActixUri;
use veruna_domain::nodes::{Node, NodeKitFactory};

#[derive(Clone)]
pub struct AppState {
    repositories: Arc<dyn veruna_domain::input::Repositories>,
}

async fn path_test(request: HttpRequest,
                   app: Data<AppState>) -> impl Responder
{
    let repo = app.repositories.site().await;
    let site_kit = SiteKitFactory::build(repo);
    let uri_parser = uri::UriParser::new(&request);
    if uri_parser.ends_with_slash() {}
    let tail: String = request.match_info().get("tail").unwrap().parse().unwrap();
    let url = uri_parser.parse();
    let site = site_kit.get_site(url).await;
    match site {
        None => {
            HttpResponse::from_error(InternalError::new(
                format!("not found site for root node"),
                StatusCode::NOT_FOUND,
            ))
        }
        Some(n) => {
            let node_kit = NodeKitFactory::build_node_kit(
                app.repositories.nodes().await
            );
            let path = node_kit
                .find_path(uri_parser.path.clone())
                .await;
            match path {
                None => {
                    response::NotFound::new(
                        format!("not found node {}", uri_parser.path.clone())
                    )
                }
                Some(node) => {
                    let view = MainPageView {
                        title: format!("{}, {}", uri_parser.path, tail),
                        site: view::Site {
                            name: n.0.name().to_string(),
                            description: n.0.description().to_string(),
                        },
                        node: NodeView { title: node.title(), path: node.path() },
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
            .service(
                Fs::new("/static/", "./static/")
                .index_file("index.html"))
            .route("{tail:.*}", web::get().to(path_test))
            .app_data(Data::new(state.clone()))
    };

    HttpServer::new(app_factory)
        .bind(("127.0.0.1", 20921))?
        .run()
        .await
}