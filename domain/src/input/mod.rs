use std::sync::{Arc};
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::nodes::NodesRepository;
use crate::roles::RolesRepository;
use crate::sites::SiteRepository;
use crate::users::UsersRepository;

#[async_trait(? Send)]
pub trait Repositories : Send + Sync {
    fn site(&self) -> Box<dyn SiteRepository>;
    async fn nodes(&self) -> Box<dyn NodesRepository>;
    fn users(&self) -> Arc<Mutex<dyn UsersRepository>>;
    fn roles(&self) -> Arc<Mutex<dyn RolesRepository>>;
}