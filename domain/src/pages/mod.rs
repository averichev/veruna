use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::{DataErrorTrait, DomainEntryTrait, DomainErrorTrait};
use crate::sites::SiteId;

#[async_trait(? Send)]
pub trait PageRepositoryTrait {
    async fn create(&self, page: Arc<dyn PageCreateTrait>) -> Result<Arc<dyn PageTrait>, Arc<dyn DataErrorTrait>>;
    async fn read_without_parent(&self) -> Result<Option<Arc<dyn PageTrait>>, Arc<dyn DataErrorTrait>>;
    async fn read_by_parent(&self, code: String, parent: PageId) -> Result<Option<Arc<dyn PageTrait>>, Arc<dyn DataErrorTrait>>;
    async fn delete(&self, page_id: PageId) -> bool;
    async fn list(&self) -> Result<Arc<Vec<Box<dyn PageTrait>>>, Arc<dyn DataErrorTrait>>;
}

#[async_trait(? Send)]
pub trait PageKitTrait {
    async fn create(&self, page: Arc<dyn PageCreateTrait>) -> Result<Arc<dyn PageTrait>, Arc<dyn DomainErrorTrait>>;
    async fn read(&self, nodes: Vec<String>) -> Result<Option<Arc<dyn PageTrait>>, Arc<dyn DomainErrorTrait>>;
    async fn list(&self) -> Result<Arc<Vec<Box<dyn PageTrait>>>, Arc<dyn DomainErrorTrait>>;
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
    async fn create(&self, page: Arc<dyn PageCreateTrait>) -> Result<Arc<dyn PageTrait>, Arc<dyn DomainErrorTrait>> {
        let create = self.repository.lock().await.create(page).await.unwrap();
        Ok(create)
    }

    async fn read(&self, nodes: Vec<String>) -> Result<Option<Arc<dyn PageTrait>>, Arc<dyn DomainErrorTrait>> {
        if nodes.is_empty() {
            let read = self.repository.lock().await.read_without_parent().await.unwrap();
            return Ok(read);
        }
        let mut pages: Vec<Arc<dyn PageTrait>> = vec![];
        let main = self.repository.lock().await.read_without_parent().await.unwrap();
        match main {
            None => {
                return Err(PageError::new("главная страница не найдена".to_string()));
            }
            Some(m) => {
                pages.push(m);
            }
        }
        for (i, node) in nodes.iter().enumerate() {
            let index = i;
            let prev: &Arc<dyn PageTrait> = pages.get(index).unwrap();
            let parent = PageId { value: prev.id() };
            let read = self.repository.lock().await.read_by_parent(node.clone(), parent).await.unwrap();

            match read {
                None => {
                    return Ok(None);
                }
                Some(page) => {
                    pages.push(page)
                }
            }
        }
        let last_page = pages.last();
        Ok(last_page.cloned())
    }

    async fn list(&self) -> Result<Arc<Vec<Box<dyn PageTrait>>>, Arc<dyn DomainErrorTrait>> {
        let result = self.repository.lock().await.list().await.unwrap();
        Ok(result)
    }
}

pub trait PageCreateTrait {
    fn name(&self) -> String;
    fn code(&self) -> Option<String>;
}

pub trait PageTrait {
    fn name(&self) -> String;
    fn code(&self) -> Option<String>;
    fn id(&self) -> String;
}

pub struct Page {
    code: String,
    name: String,
}

pub struct PageId {
    pub value: String,
}

pub struct PageSite {
    page: PageId,
    site: dyn SiteId,
}

#[derive(Clone, Debug)]
struct PageError {
    message: String,
}

impl PageError {
    fn new(message: String) -> Arc<dyn DomainErrorTrait> {
        Arc::new(PageError { message })
    }
}

impl DomainErrorTrait for PageError {
    fn message(&self) -> String {
        self.message.clone()
    }
}