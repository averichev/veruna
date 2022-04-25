use serde::{Deserialize};

#[derive(Clone, Debug, Deserialize)]
pub enum Type{
    #[serde(rename(deserialize = "header", serialize = "header"))]
    Header,
    #[serde(rename(deserialize = "paragraph", serialize = "paragraph"))]
    Paragraph
}