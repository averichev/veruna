use async_trait::async_trait;
use crate::nodes::NodesRepository;
use crate::sites::SiteRepository;
use crate::users::UsersRepository;

#[async_trait(? Send)]
pub trait Repositories : Send + Sync {
    async fn site(&self) -> Box<dyn SiteRepository>;
    async fn nodes(&self) -> Box<dyn NodesRepository>;
    async fn users(&self) -> Box<dyn UsersRepository>;
}