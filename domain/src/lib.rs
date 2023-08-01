#![allow(dead_code)]

use async_trait::async_trait;
use crate::input::Repositories;
use crate::sites::site_kit::{SiteKit, SiteKitFactory};

pub mod sites;
pub mod pages;
pub mod input;
pub mod nodes;
pub mod users;
pub mod roles;

#[async_trait(? Send)]
trait DomainEntry {
    async fn site_kit(&self) -> Box<dyn SiteKit>;
}

pub struct Entry {
    repositories: Box<dyn Repositories>,
}

#[async_trait(? Send)]
impl DomainEntry for Entry {
    async fn site_kit(&self) -> Box<dyn SiteKit> {
        let repositories = &self.repositories;
        let repo = repositories.site().await;
        let site_kit = SiteKitFactory::build(repo);
        site_kit
    }
}

trait DomainError {
    fn message(self) -> String;
}

pub trait DataError {
    fn message(&self) -> String;
}