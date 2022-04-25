use serde::{Serialize, Deserialize};
use crate::models::data::Data;
use crate::models::r#type::Type;

#[derive(Debug, Deserialize)]
pub struct Block {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub r#type: Type,
    pub data: Data
}

// #[derive(Debug, Deserialize)]
// pub struct Block {
//     id: Option<String>,
//     #[serde(rename(deserialize = "type", serialize = "type"))]
//     type_: BlockType,
//     data: BlockData
// }