#![allow(dead_code)]

use std::sync::Arc;
use async_trait::async_trait;
use crate::input::Repositories;
use crate::sites::site_kit::{SiteKit, SiteKitFactory};
use crate::users::{UserKit, UserKitContract};

pub mod sites;
pub mod pages;
pub mod input;
pub mod nodes;
pub mod users;
pub mod roles;

#[async_trait(? Send)]
pub trait DomainEntryTrait: Send + Sync {
    async fn site_kit(&self) -> Box<dyn SiteKit>;
    async fn user_kit(&self) -> Box<dyn UserKitContract>;
}

pub struct DomainEntry {
    repositories: Arc<dyn Repositories>,
}

impl DomainEntry {
    pub fn new(repositories: Arc<dyn Repositories>) -> Arc<dyn DomainEntryTrait> {
        Arc::new(DomainEntry { repositories })
    }
}

#[async_trait(? Send)]
impl DomainEntryTrait for DomainEntry {
    async fn site_kit(&self) -> Box<dyn SiteKit> {
        let repositories = &self.repositories;
        let repo = repositories.site().await;
        let site_kit = SiteKitFactory::build(repo);
        site_kit
    }

    async fn user_kit(&self) -> Box<dyn UserKitContract> {
        let repositories = &self.repositories;
        let repository = repositories.users().await;
        Box::new(UserKit { repository })
    }
}

pub trait DomainError {
    fn message(&self) -> String;
}

pub trait DataError {
    fn message(&self) -> String;
}