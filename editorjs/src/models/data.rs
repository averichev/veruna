use std::borrow::Borrow;
use std::fmt;
use std::fmt::{Debug, Formatter, Write};
use std::ops::Deref;
use serde::{Serialize, Deserialize, Deserializer, de};
use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, Unexpected};
use serde_json::error::Error as JsonError;

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