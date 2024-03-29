pub(crate) mod node;
pub(crate) mod site;
mod users;
mod role;
pub mod migration;
mod page;

use std::ops::Deref;
use std::sync::{Arc,};
use async_trait::async_trait;
use veruna_domain::sites::{SiteTrait, SiteBuilder, SiteId, SiteRepositoryTrait as SiteRepositoryContract, SiteRepositoryTrait};
use veruna_domain::{RecordId as RecordIdTrait, DataErrorTrait as DataErrorTrait};
use veruna_domain::nodes::NodesRepository;
use std::borrow::Borrow;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use tokio::sync::Mutex;
use veruna_domain::pages::PageRepositoryTrait;
use veruna_domain::roles::RolesRepository as RolesRepositoryTrait;
use crate::page::PageRepository;
use crate::site::SiteRepository;
use crate::users::UsersRepository;
use crate::role::RolesRepository;


pub struct Repositories {
    connection: Arc<Surreal<Db>>,
}

impl Repositories {
    pub fn new(connection: Arc<Surreal<Db>>) -> Arc<dyn veruna_domain::input::Repositories> {
        Arc::new(Repositories {
            connection
        })
    }
}

#[async_trait(? Send)]
impl veruna_domain::input::Repositories for Repositories {
    fn site(&self) -> Arc<Mutex<dyn SiteRepositoryTrait>> {
        SiteRepository::new(self.connection.clone())
    }

    async fn nodes(&self) -> Box<dyn NodesRepository> {
        node::NodesRepositoryImpl::new(self.connection.clone()).await
    }

    fn users(&self) -> Arc<Mutex<dyn veruna_domain::users::UsersRepository>> {
        UsersRepository::new(self.connection.clone())
    }

    fn roles(&self) -> Arc<Mutex<dyn RolesRepositoryTrait>> {
        RolesRepository::new(self.connection.clone())
    }

    fn pages(&self) -> Arc<Mutex<dyn PageRepositoryTrait>> {
        PageRepository::new(self.connection.clone())
    }
}


pub(crate) struct RecordId {
    value: String,
}

impl RecordId {
    fn new(value: String) -> RecordId {
        RecordId { value }
    }
}

impl RecordIdTrait for RecordId {
    fn value(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug)]
pub(crate) struct DataError {
    message: String,
}

impl DataError {
    fn new(message: String) -> DataError {
        DataError { message }
    }
}

impl DataErrorTrait for DataError {
    fn message(&self) -> String {
        self.message.clone()
    }
}