pub mod site_kit;

use std::sync::Arc;
use crate::pages::PageId;
use async_trait::async_trait;
use serde::Serialize;
use serde::Deserialize;
use crate::DataErrorTrait;
use crate::sites::site_kit::SiteKitTrait;

#[async_trait(? Send)]
pub trait SiteRepositoryTrait {
    async fn create(&mut self, site: Box<dyn SiteTrait>) -> Box<dyn SiteId>;
    async fn read(&self, read_by: SiteReadOption) -> Result<Option<Arc<dyn SiteTrait>>, Arc<dyn DataErrorTrait>>;
    async fn delete(&self, site_id: Box<dyn SiteId>) -> bool;
    async fn list(&self) -> Arc<Vec<Box<dyn SiteTrait>>>;
}

pub trait CreatedSite {
    fn site(&self) -> dyn SiteTrait;
    fn site_id(&self) -> Box<dyn SiteId>;
}

pub trait SiteIdBuilder {
    fn build(&self, id: String) -> Box<dyn SiteId>;
}

pub struct SiteIdBuilderImpl;

impl SiteIdBuilderImpl {
    pub fn new() -> Box<dyn SiteIdBuilder> {
        let result: Box<dyn SiteIdBuilder> = Box::new(SiteIdBuilderImpl {});
        result
    }
}

impl SiteIdBuilder for SiteIdBuilderImpl {
    fn build(&self, id: String) -> Box<dyn SiteId> {
        let result = SiteIdImpl { value: id };
        let b: Box<dyn SiteId> = Box::new(result);
        b
    }
}


pub trait Reader {
    fn read(&self, site_id: Box<dyn SiteId>) -> Box<dyn SiteTrait>;
}

pub struct SiteReader<'a> {
    site_repository: &'a Box<dyn SiteRepositoryTrait>,
}

impl SiteReader<'_> {
    fn new(site_repository: &Box<dyn SiteRepositoryTrait>) -> Box<dyn Reader + '_> {
        Box::new(SiteReader { site_repository })
    }
}

impl Reader for SiteReader<'_> {
    fn read(&self, site_id: Box<dyn SiteId>) -> Box<dyn SiteTrait> {
        todo!()
    }
}


pub trait SiteBuilder {
    fn build(&self, site: Site) -> Box<dyn SiteTrait>;
}

pub struct SiteBuilderImpl;

impl SiteBuilderImpl {
    pub fn new() -> Box<dyn SiteBuilder> {
        let result: Box<dyn SiteBuilder> = Box::new(SiteBuilderImpl {});
        result
    }
}

impl SiteBuilder for SiteBuilderImpl {
    fn build(&self, site: Site) -> Box<dyn SiteTrait> {
        let result: Box<dyn SiteTrait> = Box::new(site);
        result
    }
}

pub enum SiteReadOption {
    SiteId(Box<dyn SiteId>),
    Domain(String),
}

pub trait SiteTrait {
    fn id(&self) -> String;
    fn domain(&self) -> String;
    fn name(&self) -> String;
    fn description(&self) -> String;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Site {
    pub id: String,
    pub domain: String,
    pub name: String,
    pub description: String,
}

impl Site {
    pub fn new(domain: String, name: String, description: String, id: String) -> Box<dyn SiteTrait> {
        let site = Site {
            id,
            domain,
            name,
            description,
        };
        Box::new(site)
    }
}

impl SiteTrait for Site {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn domain(&self) -> String {
        self.domain.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}

pub trait SiteId {
    fn value(&self) -> String;
}

struct SiteIdImpl {
    value: String,
}

impl SiteId for SiteIdImpl {
    fn value(&self) -> String {
        self.value.clone()
    }
}

pub struct SitePages {
    pages: Vec<PageId>,
}