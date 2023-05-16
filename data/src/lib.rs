use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, DbConn, DbErr, NotSet};
use sea_orm::ActiveValue::Set;
use sea_orm::{entity::*, query::*};
use veruna_domain::sites::{Site, SiteBuilder, SiteBuilderImpl, SiteId, SiteIdBuilderImpl, SiteReadOption, SiteRepository as SiteRepositoryContract};
use entity::site::{ActiveModel, Entity, Model};

struct SiteRepository {
    sites: HashMap<u8, Box<dyn Site>>,
    connection: DbConn,
}

impl SiteRepository {
    pub async fn new(database_url: &String) -> Box<dyn SiteRepositoryContract> {
        let connection = Database::connect(database_url)
            .await
            .expect("Failed to setup the database");
        let result = SiteRepository { sites: Default::default(), connection };
        Box::new(result)
    }
}

#[async_trait(? Send)]
impl SiteRepositoryContract for SiteRepository {
    async fn create(&mut self, site: Box<dyn Site>) -> Box<dyn SiteId> {
        let new_site = ActiveModel {
            id: NotSet,
            name: NotSet,
            domain: Set(site.domain()),
            port: NotSet,
        };
        let result = new_site
            .save(&self.connection)
            .await;
        let builder = SiteIdBuilderImpl::new();
        match result {
            Ok(n) => {
                builder.build(n.id.unwrap() as u8)
            }
            Err(e) => {
                print!("{}", e.to_string());
                e.to_string();
                builder.build(42)
            }
        }
    }

    async fn read(&self, read_by: SiteReadOption) -> Option<(Box<dyn Site>, Box<dyn SiteId>)> {
        match read_by {
            SiteReadOption::SiteId(id) => {
                None
            }
            SiteReadOption::Domain(domain) => {
                println!("{}", domain);
                let site = Entity::find()
                    .filter(<entity::prelude::Site as EntityTrait>::Column::Domain.eq(domain))
                    .one(&self.connection)
                    .await;
                match site {
                    Ok(s) => {
                        match s {
                            None => {
                                None
                            }
                            Some(ss) => {
                                let builder = SiteIdBuilderImpl::new();
                                let site_id = builder.build(ss.id);
                                let site_builder = SiteBuilderImpl::new();
                                let site = site_builder.build();
                                Some((site, site_id))
                            }
                        }
                    }
                    Err(_) => {
                        None
                    }
                }
            }
        }
    }

    fn delete(&self, site_id: Box<dyn Site>) -> bool {
        todo!()
    }
}

pub struct Repositories {
    database_url: String,
}

impl Repositories {
    pub fn new(database_url: String) -> Arc<dyn veruna_domain::input::Repositories> {
        Arc::new(Repositories { database_url })
    }
}

#[async_trait(? Send)]
impl veruna_domain::input::Repositories for Repositories {
    async fn site(&self) -> Box<dyn SiteRepositoryContract> {
        let database_url = &self.database_url;
        SiteRepository::new(database_url).await
    }
}