use std::collections::HashMap;
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::{Error, Surreal};
use veruna_domain::sites::{Site, SiteId, SiteIdBuilder, SiteIdBuilderImpl, SiteImpl, SiteReadOption, SiteRepository};
use async_trait::async_trait;
use serde::Deserialize;
use surrealdb::sql::Thing;
use crate::SiteRepositoryContract;

pub(crate) struct SiteRepositoryImpl {
    sites: HashMap<u8, Box<dyn Site>>,
    connection: Arc<Surreal<Db>>,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

impl SiteRepositoryImpl {
    pub async fn new(connection: Arc<Surreal<Db>>) -> Box<dyn SiteRepositoryContract> {
        let result = SiteRepositoryImpl { sites: Default::default(), connection };
        Box::new(result)
    }
    async fn insert_site(&self) -> Result<Thing, Error> {
        let record: Record = self.connection
            .create("sites")
            .content(SiteImpl {
                domain: "localhost".to_string(),
                name: "Test site".to_string(),
                description: "Test site".to_string(),
            })
            .await?;
        Ok(record.id)
    }
    async fn find_site_by_domain(&self, domain: String) -> Result<Option<SiteImpl>, Error> {
        let mut response = self.connection
            .query("SELECT * FROM sites WHERE domain = $domain")
            .bind(("domain", domain))
            .await?;
        let sites: Option<SiteImpl> = response.take(0)?;
        //         let record = sites.get(0).unwrap().clone();
        //         let site_id = SiteIdBuilderImpl::new().build("dasdasd".to_string());
        Ok(Some(sites.unwrap().clone()))
    }
    async fn count_sites(&self) -> Result<usize, Error> {
        let db = self.connection.clone();
        let mut all_sites_response = db.query("SELECT count() AS count FROM sites").await?;
        let take = all_sites_response.take::<Option<usize>>("count")?;
        match take {
            Some(n) => {
                Ok(n)
            }
            None => {
                Ok(0)
            }
        }
        // match read_by {
        //     SiteReadOption::SiteId(_) => {
        //         None
        //     }
        //     SiteReadOption::Domain(domain) => {
        //
        //         db.use_ns("test").use_db("test").await.unwrap();
        //         db.set("site", SiteImpl {
        //             domain,
        //             name: "".to_string(),
        //             description: "".to_string(),
        //         }).await.ok();
        //         let mut all_sites_response = db.query("SELECT count() AS count FROM Sites").await.unwrap();
        //         all_sites_response.take(0);
        //         // if count.unwrap() == 0 {
        //         //     let record: surrealdb::sql::Kind = db
        //         //         .create("Sites")
        //         //         .content(SiteImpl {
        //         //             domain: "localhost".to_string(),
        //         //             name: "Test site".to_string(),
        //         //             description: "Test site".to_string(),
        //         //         })
        //         //         .await
        //         //         .unwrap();
        //         // }
        //         let mut response = db.query("SELECT * FROM Sites WHERE domain = site.domain").await.unwrap();
        //         let sites: Vec<SiteImpl> = response.take(0).unwrap();
        //         let record = sites.get(0).unwrap().clone();
        //         let site_id = SiteIdBuilderImpl::new().build("dasdasd".to_string());
        //         Some((Arc::new(record), site_id))
        //     }
        // }
    }
}

#[async_trait(? Send)]
impl SiteRepository for SiteRepositoryImpl {
    async fn create(&mut self, site: Box<dyn Site>) -> Box<dyn SiteId> {
        todo!()
    }

    async fn read(&self, read_by: SiteReadOption) -> Option<(Arc<dyn Site>, Box<dyn SiteId>)> {
        let count_sites = self.count_sites().await;
        match count_sites {
            Ok(count) => {
                if count == 0 {
                    self.insert_site().await.unwrap();
                }
                let site = self.find_site_by_domain("localhost".to_string())
                    .await
                    .unwrap();
                match site {
                    None => {
                        None
                    }
                    Some(n) => {
                        Some((Arc::new(n), SiteIdBuilderImpl.build("dasdasd".to_string())))
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }

    fn delete(&self, site_id: Box<dyn Site>) -> bool {
        todo!()
    }
}