use std::sync::{Arc};
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::nodes::NodesRepository;
use crate::pages::PageRepositoryTrait;
use crate::roles::RolesRepository;
use crate::sites::SiteRepositoryTrait;
use crate::users::UsersRepository;

#[async_trait(? Send)]
pub trait Repositories : Send + Sync {
    fn site(&self) -> Arc<Mutex<dyn SiteRepositoryTrait>>;
    async fn nodes(&self) -> Box<dyn NodesRepository>; // удалить
    fn users(&self) -> Arc<Mutex<dyn UsersRepository>>;
    fn roles(&self) -> Arc<Mutex<dyn RolesRepository>>;
    fn pages(&self) -> Arc<Mutex<dyn PageRepositoryTrait>>;
}