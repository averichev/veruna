#![feature(inherent_associated_types)]

mod uri;
mod view;
mod response;
mod policy;
mod handlers;
mod middleware;
mod errors;
mod models;

use std::env;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use actix_cors::Cors;
use veruna_domain::sites::site_kit::SiteKitFactory;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_web::http::StatusCode;
use actix_web::error::InternalError;
use actix_web::web::Data;
use actix_files::{Files};
use futures_util::FutureExt;
use actix_web::middleware::Logger;
use actix_web::{
    dev::{Service, Transform}
};
use actix_web_validator::JsonConfig;
use dotenv;
use veruna_data::Repositories;
use veruna_domain::sites::{Site, SiteId};
use crate::view::{MainPageView, NodeView};
use async_trait::async_trait;
use sailfish::TemplateOnce;
use veruna_domain::nodes::{Node, NodeKitFactory};
use casbin::{Adapter, CoreApi, DefaultModel, Enforcer, Filter, Model};
use log::log;
use serde::Serialize;
use surrealdb::engine::local::{Db, File};
use surrealdb::opt::Strict;
use surrealdb::Surreal;
use termion::{color, style};
use veruna_domain::{DomainEntry, DomainEntryTrait};
use crate::handlers::error_handler::ValidationErrorJsonPayload;
use crate::middleware::admin_api::AdminApi;
use crate::middleware::redirect_slash::{RedirectSlash};
use crate::middleware::static_admin::SayHi;
use crate::models::{CurrentUser, CurrentUserTrait};
use crate::policy::Policy;

#[derive(Clone)]
pub struct AppState {
    repositories: Arc<dyn veruna_domain::input::Repositories>,
    connection: Arc<Surreal<Db>>,
    instance_code: String,
    domain: Arc<dyn DomainEntryTrait>,
}

async fn path_test(request: HttpRequest,
                   app: Data<AppState>) -> impl Responder
{
    let repo = app.repositories.site();
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
        for item in policy {
            m.add_policy("p", "p", vec![item.rule, item.object, item.action]);
        }
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
            .body("admin api")
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
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let db: Surreal<Db> = Surreal::new::<File>((&*db_url(), Strict)).await.unwrap();
    db.use_ns("veruna").use_db("veruna").await.unwrap();
    let connection = Arc::new(db);
    veruna_data::migration::Migration::start(&connection).await;
    let repo = Repositories::new(connection.clone());
    let instance_code = uuid7::uuid7().to_string();
    let domain = DomainEntry::new(repo.clone());
    println!("{}{}{}{}{}",
             style::Bold,
             color::Fg(color::Green),
             instance_code,
             style::Reset,
             color::Fg(color::Reset)
    );
    let args: Vec<String> = env::args().collect();
    match args.len() {
        4 => {
            return match args[1].as_str() {
                "admin" => {
                    match args[2].as_str() {
                        "add" => {
                            let username = args[3].as_str();
                            eprintln!("добавляем {}", username);
                            domain.user_kit().create_admin(username.to_string()).await;
                            Ok(())
                        }
                        _ => {
                            eprintln!("Неизвестная подкоманда");
                            Ok(())
                        }
                    }
                }
                _ => {
                    eprintln!("Неизвестная команда");
                    Ok(())
                }
            };
        }
        _ => {}
    }

    let state = AppState {
        repositories: repo.clone(),
        connection: connection.clone(),
        instance_code,
        domain: domain.clone(),
    };

    let current_user = CurrentUser::new();
    let current_user_arc: Arc<Mutex<dyn CurrentUserTrait>> = Arc::new(Mutex::new(current_user.clone()));
    let data = Data::new(current_user_arc.clone());

    log::info!("starting HTTP server at http://localhost:20921");

    let app_factory = move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(JsonConfig::default().error_handler(|err, _| {
                let json_error = match &err {
                    actix_web_validator::Error::Validate(error) => ValidationErrorJsonPayload::from(error),
                    _ => ValidationErrorJsonPayload { errors: Vec::new() },
                };
                InternalError::from_response(err, HttpResponse::UnprocessableEntity().json(json_error)).into()
            }))
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(RedirectSlash)
            .service(
                web::scope("/api/protected")
                    .wrap(AdminApi::new(current_user_arc.clone()))
                    .route("get-current-user/", web::post().to(handlers::get_current_user::handle_form_data))
            )
            .route("/login/", web::post().to(handlers::login::handle_form_data))
            .route("/register/", web::post().to(handlers::register::register_action))
            .route("/admin/site/list/", web::get().to(handlers::admin::sites::sites_list))
            .service(
                web::scope("/static/admin")
                    .wrap(SayHi)
                    .service(Files::new("/", "./static/admin")
                        .index_file("index.html")
                        .show_files_listing()
                    )
            )
            .route("/admin/", web::get().to(admin))
            .route("{tail:.*}", web::get().to(path_test))
            .app_data(Data::new(state.clone()))
            .app_data(Data::clone(&data))
    };

    HttpServer::new(app_factory)
        .bind(("127.0.0.1", 20921))?
        .run()
        .await
}



