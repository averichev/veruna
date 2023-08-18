use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::{DataError, DomainError};
use crate::sites::SiteId;

#[async_trait(? Send)]
pub trait PageRepositoryTrait {
    async fn list(&self) -> Result<Arc<Vec<Box<dyn PageTrait>>>, Arc<dyn DataError>>;
    async fn create(&self, page: Arc<dyn PageCreateTrait>) -> Result<Arc<dyn PageTrait>, Arc<dyn DataError>>;
    async fn delete(&self, page_id: PageId) -> bool;
}

#[async_trait(? Send)]
pub trait PageKitTrait {
    async fn list(&self) -> Result<Arc<Vec<Box<dyn PageTrait>>>, Arc<dyn DomainError>>;
    async fn create(&self, page: Arc<dyn PageCreateTrait>) -> Result<Arc<dyn PageTrait>, Arc<dyn DomainError>>;
}

pub struct PageKit {
    repository: Arc<Mutex<dyn PageRepositoryTrait>>,
}

impl PageKit {
    pub(crate) fn new(repository: Arc<Mutex<dyn PageRepositoryTrait>>) -> Arc<dyn PageKitTrait> {
        Arc::new(PageKit { repository })
    }
}

#[async_trait(? Send)]
impl PageKitTrait for PageKit {
    async fn list(&self) -> Result<Arc<Vec<Box<dyn PageTrait>>>, Arc<dyn DomainError>> {
        let result = self.repository.lock().await.list().await.unwrap();
        Ok(result)
    }

    async fn create(&self, page: Arc<dyn PageCreateTrait>) -> Result<Arc<dyn PageTrait>, Arc<dyn DomainError>> {
        let create = self.repository.lock().await.create(page).await.unwrap();
        Ok(create)
    }
}

pub trait PageCreateTrait {
    fn name(&self) -> String;
    fn code(&self) -> String;
}

pub trait PageTrait {
    fn name(&self) -> String;
    fn code(&self) -> String;
    fn id(&self) -> String;
}

pub struct Page {
    code: String,
    name: String,
}

pub struct PageId {
    value: String,
}

pub struct PageSite {
    page: PageId,
    site: dyn SiteId,
}