#![allow(dead_code)]

use std::fmt;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use crate::input::Repositories;
use crate::sites::site_kit::{SiteKit, SiteKitFactory};
use crate::users::{UserKit, UserKitContract};
use crate::users::events::{UserEventsContainer};

pub mod sites;
pub mod pages;
pub mod input;
pub mod nodes;
pub mod users;
pub mod roles;

#[async_trait(? Send)]
pub trait DomainEntryTrait: Send + Sync {
    async fn site_kit(&self) -> Box<dyn SiteKit>;
    fn user_kit(&self) -> Box<dyn UserKitContract>;
}

pub struct DomainEvents {
    users: Arc<UserEventsContainer>,
}

impl DomainEvents {
    pub(crate) fn new() -> DomainEvents {
        let users = UserEventsContainer::new();
        DomainEvents { users: Arc::new(users) }
    }
}

pub struct DomainEntry {
    repositories: Arc<dyn Repositories>,
    events: Arc<DomainEvents>,
}

impl DomainEntry {
    pub fn new(repositories: Arc<dyn Repositories>) -> Arc<dyn DomainEntryTrait> {
        let events = Arc::new(DomainEvents::new());
        let container = events.users.clone();
        roles::UserEventsListener::new(events.users.clone()).register_observer(container.receiver.clone());
        Arc::new(DomainEntry { repositories, events })
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

    fn user_kit(&self) -> Box<dyn UserKitContract> {
        let repositories = &self.repositories;
        let repository = repositories.users();
        let events = Arc::clone(&self.events.users);
        Box::new(UserKit::new(repository, events))
    }
}

pub trait DomainError: fmt::Debug {
    fn message(&self) -> String;
}

pub trait DataError: fmt::Debug {
    fn message(&self) -> String;
}