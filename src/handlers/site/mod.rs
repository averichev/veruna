use std::sync::Arc;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use linq::iter::Enumerable;
use serde::Serialize;
use veruna_domain::sites::SiteTrait;
use crate::AppState;

pub(crate) async fn list(app: Data<AppState>) -> impl Responder {
    let site_kit = app.domain.site_kit();
    let list = site_kit.list().await;
    let list = SiteList::new(list);
    HttpResponse::Ok().json(
        SiteListResponse{ list }
    )
}

#[derive(Serialize)]
struct SiteListResponse {
    list: SiteList
}


#[derive(Serialize)]
struct SiteList {
    items: Vec<SiteListItem>,
}

impl SiteList {
    fn new(list: Arc<Vec<Box<dyn SiteTrait>>>) -> SiteList {
        let items = list.iter()
            .select(|n| SiteListItem {
                id: n.id(),
                name: n.name(),
                description: n.description(),
                domain: n.domain(),
            })
            .collect();
        SiteList { items }
    }
}

#[derive(Serialize)]
struct SiteListItem {
    id: String,
    name: String,
    description: String,
    domain: String,
}