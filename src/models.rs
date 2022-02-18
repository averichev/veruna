use serde::Serialize;

#[derive(Serialize)]
#[derive(Queryable)]
pub struct Page {
    pub id: i32,
    pub name: String
}

#[derive(Serialize)]
#[derive(Queryable)]
pub struct PageItem {
    pub id: i32,
    pub name: String,
}