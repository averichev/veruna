use std::fmt::{Debug};
use serde::{Deserialize};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Data {
    #[serde(rename = "header")]
    Header {
        text: String,
        level: i8
    },
    #[serde(rename = "paragraph")]
    Paragraph {
        text: String
    }
}