use serde::Serialize;

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct Page {
    pub id: i32,
    pub name: String
}

#[derive(Queryable, Serialize)]
pub struct PageItem {
    pub id: i32,
    pub name: String,
}