use std::sync::Arc;
use async_trait::async_trait;
use url::{ParseError, Url};
use log;
use log::info;
use tokio::sync::Mutex;
use crate::DomainErrorTrait;
use crate::sites::{Reader, SiteTrait, SiteBuilder, SiteBuilderImpl, SiteId, SiteIdBuilder, SiteIdBuilderImpl, SiteReader, SiteReadOption, SiteRepositoryTrait};

#[async_trait(? Send)]
pub trait SiteKitTrait {
    async fn create(&mut self, site: Box<dyn SiteTrait>) -> Box<dyn SiteId>;
    async fn get_site(&self, url: Url) -> Result<Option<Arc<dyn SiteTrait>>, Arc<dyn DomainErrorTrait>>;
    fn site_id_builder(&self) -> Box<dyn SiteIdBuilder>;
    fn site_builder(&self) -> Box<dyn SiteBuilder>;
    async fn list(&self) -> Arc<Vec<Box<dyn SiteTrait>>>;
}

struct SiteKit {
    site_repository: Arc<Mutex<dyn SiteRepositoryTrait>>,
}

#[async_trait(? Send)]
impl SiteKitTrait for SiteKit {
    async fn create(&mut self, site: Box<dyn SiteTrait>) -> Box<dyn SiteId> {
        let result = self.site_repository.lock().await.create(site).await;
        result
    }

    async fn get_site(&self, url: Url) -> Result<Option<Arc<dyn SiteTrait>>, Arc<dyn DomainErrorTrait>> {
        let domain = url.host().unwrap().to_string();
        let site = self.site_repository.lock().await
            .read(SiteReadOption::Domain(domain))
            .await
            .unwrap();
        Ok(site)
    }

    fn site_id_builder(&self) -> Box<dyn SiteIdBuilder> {
        let result = SiteIdBuilderImpl::new();
        result
    }

    fn site_builder(&self) -> Box<dyn SiteBuilder> {
        let result = SiteBuilderImpl::new();
        result
    }

    async fn list(&self) -> Arc<Vec<Box<dyn SiteTrait>>> {
        let list = self.site_repository.lock().await.list().await;
        list
    }
}

pub struct SiteKitFactory;

impl SiteKitFactory {
    pub fn build(repo: Arc<Mutex<dyn SiteRepositoryTrait>>) -> Box<dyn SiteKitTrait> {
        let result: Box<dyn SiteKitTrait> = Box::new(SiteKit { site_repository: repo });
        result
    }
}