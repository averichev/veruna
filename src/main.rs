mod uri;
mod view;
mod response;
mod policy;

use std::env;
use std::ops::Deref;
use std::sync::Arc;
use veruna_domain::sites::site_kit::SiteKitFactory;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest, Error};
use actix_web::http::StatusCode;
use actix_web::error::InternalError;
use actix_web::web::Data;
use actix_files::{Files};
use actix_web::http::header::{HeaderValue, WWW_AUTHENTICATE};
use futures_util::FutureExt;
use actix_web::middleware::Logger;
use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}
};
use futures_util::future::LocalBoxFuture;
use dotenv;
use veruna_data::Repositories;
use veruna_domain::sites::{Site, SiteId};
use crate::view::{MainPageView, NodeView};
use actix_web::web::Redirect;
use sailfish::TemplateOnce;
use veruna_domain::nodes::{Node, NodeKitFactory};
use casbin::{Adapter, CoreApi, DefaultModel, Enforcer, Filter, Model};
use log::log;
use sea_orm::prelude::async_trait::async_trait;
use serde::Serialize;
use surrealdb::engine::local::{Db, File};
use surrealdb::Surreal;
use crate::policy::Policy;

#[derive(Clone)]
pub struct AppState {
    repositories: Arc<dyn veruna_domain::input::Repositories>,
    connection: Arc<Surreal<Db>>,
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


#[derive(Clone)]
struct DatabaseAdapter {
    connection: Arc<Surreal<Db>>,
}

impl DatabaseAdapter {
    fn new(connection: Arc<Surreal<Db>>) -> DatabaseAdapter {
        DatabaseAdapter { connection }
    }
}

#[async_trait]
impl Adapter for DatabaseAdapter {
    async fn load_policy(&self, m: &mut dyn Model) -> casbin::Result<()> {
        let mut response = self.connection.query("SELECT * FROM policy").await.unwrap();
        let policy = response.take::<Vec<Policy>>(0).unwrap();


        m.add_policy("p", "p", vec!["r.sub.age > 18 && r.sub.age < 60".to_string(), "/data1".to_string(), "read".to_string()]);
        m.add_policy("p", "p", vec!["r.sub.role == \"admin\" && r.sub.age < 60".to_string(), "/admin/*".to_string(), "read".to_string()]);
        Ok(())
    }

    async fn load_filtered_policy<'a>(&mut self, m: &mut dyn Model, f: Filter<'a>) -> casbin::Result<()> {
        todo!()
    }

    async fn save_policy(&mut self, m: &mut dyn Model) -> casbin::Result<()> {
        todo!()
    }

    async fn clear_policy(&mut self) -> casbin::Result<()> {
        todo!()
    }

    fn is_filtered(&self) -> bool {
        todo!()
    }

    async fn add_policy(&mut self, sec: &str, ptype: &str, rule: Vec<String>) -> casbin::Result<bool> {
        Ok(true)
    }

    async fn add_policies(&mut self, sec: &str, ptype: &str, rules: Vec<Vec<String>>) -> casbin::Result<bool> {
        todo!()
    }

    async fn remove_policy(&mut self, sec: &str, ptype: &str, rule: Vec<String>) -> casbin::Result<bool> {
        todo!()
    }

    async fn remove_policies(&mut self, sec: &str, ptype: &str, rules: Vec<Vec<String>>) -> casbin::Result<bool> {
        todo!()
    }

    async fn remove_filtered_policy(&mut self, sec: &str, ptype: &str, field_index: usize, field_values: Vec<String>) -> casbin::Result<bool> {
        todo!()
    }
}

async fn admin(request: HttpRequest,
               app: Data<AppState>) -> impl Responder
{
    let m = DefaultModel::from_file("rbac/rbac_model.conf")
        .await
        .unwrap();

    let e = Enforcer::new(m, DatabaseAdapter::new(app.connection.clone())).await.unwrap();

    #[derive(Serialize, Hash)]
    struct User {
        age: usize,
        role: String,
    }

    let enforce = e.enforce((User { age: 30, role: "admin".to_string() }, "/admin/t", "read")).unwrap();

    if enforce {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body("admin page")
    } else {
        response::Forbidden::new("Доступ запрещен".to_string())
    }
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
    let db: Surreal<Db> = Surreal::new::<File>(&*db_url()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    let connection = Arc::new(db);
    let repo = Repositories::new(connection.clone()).await;
    let state = AppState { repositories: repo, connection: connection.clone() };
    log::info!("starting HTTP server at http://localhost:20921");

    let app_factory = move || {
        App::new()
            .wrap(Logger::default())
            .route("/static", web::to(|| async { Redirect::to("/static/") }))
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