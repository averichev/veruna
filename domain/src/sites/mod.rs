pub mod site_kit;

use crate::pages::PageId;
use async_trait::async_trait;

#[async_trait(? Send)]
pub trait SiteRepository {
    async fn create(&mut self, site: Box<dyn Site>) -> Box<dyn SiteId>;
    async fn read(&self, read_by: SiteReadOption) -> Option<(Box<dyn Site>, Box<dyn SiteId>)>;
    fn delete(&self, site_id: Box<dyn Site>) -> bool;
}

pub trait CreatedSite {
    fn site(&self) -> dyn Site;
    fn site_id(&self) -> Box<dyn SiteId>;
}

pub trait SiteIdBuilder {
    fn build(&self, id: u8) -> Box<dyn SiteId>;
}

pub struct SiteIdBuilderImpl;

impl SiteIdBuilderImpl {
    pub fn new() -> Box<dyn SiteIdBuilder> {
        let result: Box<dyn SiteIdBuilder> = Box::new(SiteIdBuilderImpl {});
        result
    }
}

impl SiteIdBuilder for SiteIdBuilderImpl {
    fn build(&self, id: u8) -> Box<dyn SiteId> {
        let result = SiteIdImpl { value: id };
        let b: Box<dyn SiteId> = Box::new(result);
        b
    }
}


pub trait Reader {
    fn read(&self, site_id: Box<dyn SiteId>) -> Box<dyn Site>;
}

pub struct SiteReader<'a> {
    site_repository: &'a Box<dyn SiteRepository>,
}

impl SiteReader<'_> {
    fn new(site_repository: &Box<dyn SiteRepository>) -> Box<dyn Reader + '_> {
        Box::new(SiteReader { site_repository })
    }
}

impl Reader for SiteReader<'_> {
    fn read(&self, site_id: Box<dyn SiteId>) -> Box<dyn Site> {
        todo!()
    }
}


pub trait SiteBuilder {
    fn build(&self, site: SiteImpl) -> Box<dyn Site>;
}

pub struct SiteBuilderImpl;

impl SiteBuilderImpl {
    pub fn new() -> Box<dyn SiteBuilder> {
        let result: Box<dyn SiteBuilder> = Box::new(SiteBuilderImpl {});
        result
    }
}

impl SiteBuilder for SiteBuilderImpl {
    fn build(&self, site: SiteImpl) -> Box<dyn Site> {
        let result: Box<dyn Site> = Box::new(site);
        result
    }
}

pub enum SiteReadOption {
    SiteId(Box<dyn SiteId>),
    Domain(String),
}

pub trait Site {
    fn domain(&self) -> String;
    fn name(&self) -> String;
    fn description(&self) -> String;
}

pub struct SiteImpl {
    pub domain: String,
    pub name: String,
    pub description: String,
}

impl Site for SiteImpl {
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
    fn value(&self) -> u8;
}

struct SiteIdImpl {
    value: u8,
}

impl SiteId for SiteIdImpl {
    fn value(&self) -> u8 {
        self.value
    }
}

pub struct SitePages {
    pages: Vec<PageId>,
}