use std::sync::Arc;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use linq::iter::Enumerable;
use serde::Serialize;
use crate::AppState;


pub(crate) async fn sites_list(app: Data<AppState>) -> impl Responder {
    let site_kit = app.domain.site_kit();
    let list = site_kit.list().await;
    HttpResponse::Ok().json(
        SiteListResponse::new(list)
    )
}

#[derive(Serialize)]
pub(crate) struct Site {
    domain: String,
}


#[derive(Serialize)]
pub(crate) struct SiteListResponse {
    list: Vec<Site>,
}

impl SiteListResponse {
    fn new(list: Arc<Vec<Box<dyn veruna_domain::sites::Site>>>) -> SiteListResponse {
        let result: Vec<Site> = list.iter().select(|n| Site { domain: n.domain() }).collect();
        SiteListResponse{ list: result }
    }
}

