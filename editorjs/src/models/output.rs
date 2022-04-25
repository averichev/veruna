use serde::{Serialize, Deserialize};
use crate::models::block::Block;

#[derive(Debug, Deserialize)]
pub struct Output {
    time: u128,
    pub blocks: Vec<Block>,
    version: String,
}