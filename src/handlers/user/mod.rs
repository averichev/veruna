mod models;

use std::result;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use crate::AppState;
use crate::handlers::user::models::UserList;

pub(crate) async fn list(app: Data<AppState>) -> impl Responder {
    let user_kit = app.domain.user_kit();
    let list = user_kit.get_user_list().await.unwrap();
    let result = UserList::new(list);
    HttpResponse::Ok().json(
        result
    )
}