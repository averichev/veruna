use std::sync::Arc;
use async_trait::async_trait;
use linq::iter::Enumerable;
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Db;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tokio::sync::Mutex;
use veruna_domain::DataError;
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
    async fn list(&self) -> Result<Arc<Vec<Box<dyn PageTrait>>>, Arc<dyn DataError>> {
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

    async fn create(&self, page: Arc<dyn PageCreateTrait>) -> Result<Arc<dyn PageTrait>, Arc<dyn DataError>> {
        let record: PageEntity = self.connection
            .create("pages")
            .content(CreatePageQuery::new(page))
            .await
            .unwrap();
        Ok(Page::arcing(record))
    }

    async fn delete(&self, page_id: PageId) -> bool {
        todo!()
    }
}


#[derive(Debug, Deserialize)]
struct PageEntity {
    #[serde(rename(deserialize = "id"))]
    thing: Thing,
    code: String,
    name: String,
}

#[derive(Serialize)]
pub struct CreatePageQuery {
    code: String,
    name: String,
}

impl CreatePageQuery {
    fn new(page: Arc<dyn PageCreateTrait>) -> CreatePageQuery {
        CreatePageQuery { code: page.code(), name: page.name() }
    }
}

struct Page {
    code: String,
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

    fn code(&self) -> String {
        self.code.clone()
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}