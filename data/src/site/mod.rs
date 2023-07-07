use std::collections::HashMap;
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use veruna_domain::sites::{Site, SiteId, SiteImpl, SiteReadOption, SiteRepository};
use async_trait::async_trait;
use crate::SiteRepositoryContract;

pub(crate) struct SiteRepositoryImpl {
    sites: HashMap<u8, Box<dyn Site>>,
    connection: Arc<Surreal<Db>>,
}

impl SiteRepositoryImpl {
    pub async fn new(connection: Arc<Surreal<Db>>) -> Box<dyn SiteRepositoryContract> {
        let result = SiteRepositoryImpl { sites: Default::default(), connection };
        Box::new(result)
    }
}

#[async_trait(? Send)]
impl SiteRepository for SiteRepositoryImpl {
    async fn create(&mut self, site: Box<dyn Site>) -> Box<dyn SiteId> {
        todo!()
    }

    async fn read(&self, read_by: SiteReadOption) -> Option<(Box<dyn Site>, Box<dyn SiteId>)> {
        let db: Arc<Surreal<Db>> = self.connection.clone();
        match read_by {
            SiteReadOption::SiteId(_) => {
                None
            }
            SiteReadOption::Domain(domain) => {
                db.use_ns("test").use_db("test").await.unwrap();
                db.set("site", SiteImpl {
                    domain,
                    name: "".to_string(),
                    description: "".to_string(),
                }).await.ok();
                let all_sites: Vec<SiteImpl> = db.select("Sites").await.unwrap();
                if all_sites.is_empty() {
                    let record: surrealdb::sql::Kind = db
                        .create("Sites")
                        .content(SiteImpl {
                            domain: "localhost".to_string(),
                            name: "Test site".to_string(),
                            description: "Test site".to_string(),
                        })
                        .await
                        .unwrap();
                }
                let mut response = db.query("SELECT * FROM Sites WHERE domain = site.domain").await.unwrap();
                let sites: Vec<SiteImpl> = response.take(0).unwrap();
                None
            }
        }
    }

    fn delete(&self, site_id: Box<dyn Site>) -> bool {
        todo!()
    }
}