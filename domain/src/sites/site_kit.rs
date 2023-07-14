use std::sync::Arc;
use async_trait::async_trait;
use url::{ParseError, Url};
use log;
use log::info;
use crate::sites::{Reader, Site, SiteBuilder, SiteBuilderImpl, SiteId, SiteIdBuilder, SiteIdBuilderImpl, SiteReader, SiteReadOption, SiteRepository};

#[async_trait(? Send)]
pub trait SiteKit {
    async fn create(&mut self, site: Box<dyn Site>) -> Box<dyn SiteId>;
    async fn get_site(&self, url: Result<Url, ParseError>) -> Option<(Arc<dyn Site>, Box<dyn SiteId>)>;
    fn reader(&self) -> Box<dyn Reader + '_>;
    fn site_id_builder(&self) -> Box<dyn SiteIdBuilder>;
    fn site_builder(&self) -> Box<dyn SiteBuilder>;
}

struct SiteKitImpl {
    site_repository: Box<dyn SiteRepository>,
}

#[async_trait(? Send)]
impl SiteKit for SiteKitImpl {
    async fn create(&mut self, site: Box<dyn Site>) -> Box<dyn SiteId> {
        let result = self.site_repository.create(site).await;
        result
    }

    async fn get_site(&self, url: Result<Url, ParseError>) -> Option<(Arc<dyn Site>, Box<dyn SiteId>)> {
        match url {
            Ok(u) => {
                let domain = u.host().unwrap().to_string();
                info!("{}", domain);
                let site = self.site_repository
                    .read(SiteReadOption::Domain(domain))
                    .await;
                site
            }
            Err(_) => {
                None
            }
        }
    }

    fn reader(&self) -> Box<dyn Reader + '_> {
        let repo = &self.site_repository;
        SiteReader::new(repo)
    }

    fn site_id_builder(&self) -> Box<dyn SiteIdBuilder> {
        let result = SiteIdBuilderImpl::new();
        result
    }

    fn site_builder(&self) -> Box<dyn SiteBuilder> {
        let result = SiteBuilderImpl::new();
        result
    }
}

pub struct SiteKitFactory;

impl SiteKitFactory {
    pub fn build(repo: Box<dyn SiteRepository>) -> Box<dyn SiteKit> {
        let result: Box<dyn SiteKit> = Box::new(SiteKitImpl { site_repository: repo });
        result
    }
}