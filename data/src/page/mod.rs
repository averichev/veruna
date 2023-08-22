use std::sync::Arc;
use async_trait::async_trait;
use linq::iter::Enumerable;
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Db;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tokio::sync::Mutex;
use veruna_domain::DataErrorTrait;
use veruna_domain::pages::{PageCreateTrait, PageId, PageRepositoryTrait, PageTrait};

pub(crate) struct PageRepository {
    connection: Arc<Surreal<Db>>,
}

impl PageRepository {
    pub(crate) fn new(connection: Arc<Surreal<Db>>) -> Arc<Mutex<dyn PageRepositoryTrait>> {
        Arc::new(Mutex::new(PageRepository { connection }))
    }
}

#[async_trait(? Send)]
impl PageRepositoryTrait for PageRepository {
    async fn create(&self, page: Arc<dyn PageCreateTrait>) -> Result<Arc<dyn PageTrait>, Arc<dyn DataErrorTrait>> {
        let record: PageEntity = self.connection
            .create("pages")
            .content(CreatePageQuery::new(page))
            .await
            .unwrap();
        Ok(Page::arcing(record))
    }

    async fn read_without_parent(&self) -> Result<Option<Arc<dyn PageTrait>>, Arc<dyn DataErrorTrait>> {
        let mut response = self.connection
            .query("SELECT * FROM pages WHERE code = null AND parent = null")
            .await
            .unwrap();
        let page: Option<PageEntity> = response.take(0).unwrap();
        match page {
            None => {
                Ok(None)
            }
            Some(entity) => {
                Ok(Some(Page::arcing(entity)))
            }
        }
    }

    async fn read_by_parent(&self, code: String, parent: PageId) -> Result<Option<Arc<dyn PageTrait>>, Arc<dyn DataErrorTrait>> {
        let parent_id =  format!("pages:{}", parent.value.to_string());
        let mut response = self.connection
            .query(format!("SELECT * FROM pages WHERE code = \"{}\" AND parent = \"{}\"", code, parent_id))
            .await
            .unwrap();
        let page: Option<PageEntity> = response.take(0).unwrap();
        match page {
            None => {
                Ok(None)
            }
            Some(entity) => {
                Ok(Some(Page::arcing(entity)))
            }
        }
    }

    async fn delete(&self, page_id: PageId) -> bool {
        todo!()
    }

    async fn list(&self) -> Result<Arc<Vec<Box<dyn PageTrait>>>, Arc<dyn DataErrorTrait>> {
        let mut response = self.connection
            .query("SELECT * FROM pages")
            .await
            .unwrap();
        let pages: Vec<PageEntity> = response.take(0).unwrap();
        let result: Vec<Box<dyn PageTrait>> = pages.iter()
            .select(|n| Page::boxing(n))
            .collect();
        Ok(Arc::new(result))
    }
}


#[derive(Debug, Deserialize)]
struct PageEntity {
    #[serde(rename(deserialize = "id"))]
    thing: Thing,
    code: Option<String>,
    name: String,
}

#[derive(Serialize)]
pub struct CreatePageQuery {
    code: Option<String>,
    name: String,
}

impl CreatePageQuery {
    fn new(page: Arc<dyn PageCreateTrait>) -> CreatePageQuery {
        CreatePageQuery { code: page.code(), name: page.name() }
    }
}

struct Page {
    code: Option<String>,
    name: String,
    id: String,
}

impl Page {
    fn arcing(entity: PageEntity) -> Arc<dyn PageTrait> {
        Arc::new(Page { code: entity.code, name: entity.name, id: entity.thing.id.to_string() })
    }
    fn boxing(entity: &PageEntity) -> Box<dyn PageTrait> {
        Box::new(Page { code: entity.code.clone(), name: entity.name.clone(), id: entity.thing.id.to_string() })
    }
}

impl PageTrait for Page {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn code(&self) -> Option<String> {
        self.code.clone()
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}