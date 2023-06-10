mod uri;
mod view;
mod response;

use std::{env, fs, future};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use assert_str::assert_str_eq;
use url::{ParseError, Url};
use veruna_domain::sites::site_kit::SiteKitFactory;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Error, middleware, http};
use actix_web::http::{StatusCode, Uri};
use actix_web::error::{ErrorUnauthorized, InternalError};
use actix_web::web::Data;
use actix_files::{Files};
use actix_web::body::BoxBody;
use actix_web::dev::{Response};
use actix_web::http::header::{CONTENT_TYPE, HeaderValue, LOCATION, WWW_AUTHENTICATE};
use futures_util::FutureExt;
use actix_web::http::uri::Scheme;
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}
};
use actix_web::error::ParseError::Header;
use futures_util::future::LocalBoxFuture;
use dotenv;
use veruna_data::{ConnectionBuilder, Repositories};
use veruna_domain::sites::{Site, SiteId, SiteReadOption};
use crate::view::{MainPageView, NodeView};
use actix_web::web::Redirect;
use sailfish::TemplateOnce;
use actix_web::http::Uri as ActixUri;
use veruna_domain::nodes::{Node, NodeKitFactory};
use casbin::{CoreApi, DefaultModel, Enforcer};
use log::log;
use sea_orm::Database;
use sea_orm_adapter::SeaOrmAdapter;


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

async fn admin(request: HttpRequest,
               app: Data<AppState>) -> impl Responder
{
    let m = DefaultModel::from_file("rbac/rbac_model.conf")
        .await
        .unwrap();

    let db = Database::connect(db_url())
        .await
        .unwrap();

    let a = SeaOrmAdapter::new(db).await.unwrap();
    let e = Enforcer::new(m, a).await.unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("admin page")
}

async fn redirect() -> impl Responder {
    let mut response = HttpResponse::Ok();
    response.status(StatusCode::TEMPORARY_REDIRECT);
    response.append_header((
        LOCATION,
        HeaderValue::from_static("/static/"),
    ));
    response
}

fn db_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let repo = Repositories::new(&*db_url()).await;
    let state = AppState { repositories: repo };
    log::info!("starting HTTP server at http://localhost:20921");

    let app_factory = move || {
        App::new()
            .wrap(Logger::default())
            // .service(
            //     web::scope("/static")
            //         .route("", web::to(|| async { Redirect::to("/static/") }))
            // )
            .service(
                web::scope("/static")
                    .wrap(SayHi)
                    .service(Files::new("/", "./static")
                        .index_file("index.html")
                        .show_files_listing()
                    )
            )
            .route("/admin", web::get().to(admin))
            .route("{tail:.*}", web::get().to(path_test))
            .app_data(Data::new(state.clone()))
    };

    HttpServer::new(app_factory)
        .bind(("127.0.0.1", 20921))?
        .run()
        .await
}


pub struct SayHi;

impl<S, B> Transform<S, ServiceRequest> for SayHi
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SayHiMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res: ServiceResponse<B> = fut.await?;
            res
                .headers_mut()
                .insert(
                    WWW_AUTHENTICATE,
                    HeaderValue::from_static("Basic realm=\"Restricted\""),
                );
            println!("Hi from response");
            Ok(res)
        })
    }
}