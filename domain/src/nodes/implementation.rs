use async_trait::async_trait;
use crate::nodes::{NodeKit, Node, NodesRepository};


pub(crate) struct KitStruct {
    pub(crate) repository: Box<dyn NodesRepository>,
}

#[async_trait(? Send)]
impl NodeKit for KitStruct {
    async fn find_path(&self, path: String) -> Option<Box<dyn Node>> {
        self.repository.find_path(path).await
    }
}