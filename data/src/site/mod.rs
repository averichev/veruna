use std::collections::HashMap;
use std::sync::{Arc};
use surrealdb::engine::local::Db;
use surrealdb::{Error, Surreal};
use veruna_domain::sites::{SiteTrait, SiteId, Site, SiteReadOption, SiteRepositoryTrait};
use async_trait::async_trait;
use linq::iter::Enumerable;
use serde::Deserialize;
use surrealdb::sql::Thing;
use tokio::sync::Mutex;
use crate::SiteRepositoryContract;

pub(crate) struct SiteRepository {
    sites: HashMap<u8, Box<dyn SiteTrait>>,
    connection: Arc<Surreal<Db>>,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

impl SiteRepository {
    pub fn new(connection: Arc<Surreal<Db>>) -> Arc<Mutex<dyn SiteRepositoryTrait>> {
        let result = SiteRepository { sites: Default::default(), connection };
        Arc::new(Mutex::new(result))
    }
    async fn insert_site(&self) -> Result<Thing, Error> {
        let record: Record = self.connection
            .create("sites")
            .content(Site {
                id: "".to_string(),
                domain: "localhost".to_string(),
                name: "Test site".to_string(),
                description: "Test site".to_string(),
            })
            .await?;
        Ok(record.id)
    }
    async fn find_site_by_domain(&self, domain: String) -> Result<Option<SiteEntity>, Error> {
        let mut response = self.connection
            .query("SELECT * FROM sites WHERE domain = $domain")
            .bind(("domain", domain))
            .await?;
        let site: Option<SiteEntity> = response.take(0)?;
        Ok(site)
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
impl SiteRepositoryTrait for SiteRepository {
    async fn create(&mut self, site: Box<dyn SiteTrait>) -> Box<dyn SiteId> {
        todo!()
    }

    async fn read(&self, read_by: SiteReadOption) -> Result<Option<Arc<dyn SiteTrait>>, Arc<dyn veruna_domain::DataErrorTrait>> {
        match read_by {
            SiteReadOption::SiteId(site_id) => {
                Ok(None)
            }
            SiteReadOption::Domain(domain) => {
                let site_entity = self.find_site_by_domain(domain)
                    .await
                    .unwrap();
                match site_entity {
                    None => {
                        Ok(None)
                    }
                    Some(n) => {
                        Ok(Some((Arc::new(Site {
                            id: n.thing.id.to_string(),
                            domain: n.domain,
                            name: n.name,
                            description: n.description,
                        })))
                        )
                    }
                }
            }
        }
    }

    async fn delete(&self, site_id: Box<dyn SiteId>) -> bool {
        todo!()
    }

    async fn list(&self) -> Arc<Vec<Box<dyn SiteTrait>>> {
        let mut response = self.connection
            .query("SELECT * FROM sites")
            .await
            .unwrap();
        let sites: Option<SiteEntity> = response.take(0).unwrap();
        let result: Vec<Box<dyn SiteTrait>> = sites.iter()
            .select(|n| Site::new(n.domain.to_string(), n.name.to_string(), n.description.to_string(), n.thing.id.to_string())
            )
            .collect();
        Arc::new(result)
    }
}


#[derive(Debug, Deserialize)]
struct SiteEntity {
    #[serde(rename(deserialize = "id"))]
    thing: Thing,
    domain: String,
    name: String,
    description: String,
}