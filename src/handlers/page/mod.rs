use std::sync::Arc;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use linq::iter::Enumerable;
use serde::Serialize;
use veruna_domain::pages::PageTrait;
use crate::AppState;


pub(crate) async fn list(app: Data<AppState>) -> impl Responder {
    let page_kit = app.domain.page_kit();
    let list = page_kit.list().await.unwrap();
    HttpResponse::Ok().json(
        PageListResponse { list: PageList::new(list) }
    )
}


#[derive(Serialize)]
struct PageListResponse {
    list: PageList,
}

#[derive(Serialize)]
struct PageList {
    items: Vec<PageListItem>,
}

impl PageList {
    fn new(list: Arc<Vec<Box<dyn PageTrait>>>) -> PageList {
        let items = list.iter()
            .select(|n| {
                PageListItem::new(n)
            })
            .collect();
        PageList { items }
    }
}

#[derive(Serialize)]
struct PageListItem {
    id: String,
    name: String,
    code: Option<String>,
}

impl PageListItem {
    fn new(n: &Box<dyn PageTrait>) -> PageListItem {
        PageListItem {
            id: n.id(),
            name: n.name(),
            code: n.code(),
        }
    }
}