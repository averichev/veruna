#![allow(dead_code)]

use std::fmt;
use std::sync::{Arc};
use async_trait::async_trait;
use crate::input::Repositories;
use crate::pages::{PageKit, PageKitTrait};
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
    fn site_kit(&self) -> Box<dyn SiteKit>;
    fn user_kit(&self) -> Box<dyn UserKitContract>;
    fn page_kit(&self) -> Arc<dyn PageKitTrait>;
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
        let listener = roles::UserEventsListener::new(
            events.users.clone(),
            repositories.users(),
            repositories.roles(),
        );
        listener.register_observer(container.receiver());
        Arc::new(DomainEntry { repositories, events })
    }
}

#[async_trait(? Send)]
impl DomainEntryTrait for DomainEntry {
    fn site_kit(&self) -> Box<dyn SiteKit> {
        let repositories = &self.repositories;
        let repo = repositories.site();
        let site_kit = SiteKitFactory::build(repo);
        site_kit
    }

    fn user_kit(&self) -> Box<dyn UserKitContract> {
        let repositories = &self.repositories;
        let repository = repositories.users();
        let events = Arc::clone(&self.events.users);
        Box::new(UserKit::new(repository, events))
    }

    fn page_kit(&self) -> Arc<dyn PageKitTrait> {
        let repositories = &self.repositories;
        let repository = repositories.pages();
        PageKit::new(repository)
    }
}

pub trait DomainError: fmt::Debug {
    fn message(&self) -> String;
}

pub trait DataError: fmt::Debug {
    fn message(&self) -> String;
}

pub trait RecordId {
    fn value(&self) -> String;
}