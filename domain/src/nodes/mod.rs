mod implementation;

use async_trait::async_trait;
use crate::nodes::implementation::KitStruct;

pub trait Node {
    fn new(path: String, title: String) -> Box<dyn Node> where Self: Sized;
    fn path(&self) -> String;
    fn title(&self) -> String;
}

pub struct NodeModel {
    path: String,
    title: String,
}

impl Node for NodeModel {
    fn new(path: String, title: String) -> Box<dyn Node> {
        let result: Box<dyn Node> = Box::new(NodeModel {
            path,
            title,
        });
        result
    }

    fn path(&self) -> String {
        self.path.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }
}

#[async_trait(? Send)]
pub trait NodesRepository {
    async fn find_path(&self, path: String) -> Option<Box<dyn Node>>;
}

#[async_trait(? Send)]
pub trait NodeKit {
    async fn find_path(&self, path: String) -> Option<Box<dyn Node>>;
}

pub struct NodeKitFactory;

impl NodeKitFactory {
    pub fn build_node_kit(repository: Box<dyn NodesRepository>) -> Box<dyn NodeKit> {
        Box::new(KitStruct { repository })
    }
}



