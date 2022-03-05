use serde::Serialize;
use sailfish::TemplateOnce;

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct Page {
    pub id: i32,
    pub name: String
}

#[derive(Debug, Clone, Serialize, Queryable, TemplateOnce)]
#[template(path = "product.stpl")]
pub struct Product {
    pub id: i32,
    pub name: String
}

#[derive(Queryable, Serialize)]
pub struct PageItem {
    pub id: i32,
    pub name: String,
}