use async_trait::async_trait;
use crate::nodes::{Node, NodesRepository};
use crate::sites::SiteRepository;

#[async_trait(? Send)]
pub trait Repositories : Send + Sync {
    async fn site(&self) -> Box<dyn SiteRepository>;
    async fn nodes(&self) -> Box<dyn NodesRepository>;
}