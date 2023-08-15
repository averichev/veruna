mod models;

use actix_web::{HttpResponse, Responder, web};
use actix_web::web::Data;
use crate::AppState;
use crate::handlers::user::models::{DeleteUserRequest, UserList};

pub(crate) async fn list(app: Data<AppState>) -> impl Responder {
    let user_kit = app.domain.user_kit();
    let list = user_kit.get_user_list().await.unwrap();
    let result = UserList::new(list);
    HttpResponse::Ok().json(
        result
    )
}

pub(crate) async fn delete(app: Data<AppState>, request: web::Path<DeleteUserRequest>) -> impl Responder {
    let user_kit = app.domain.user_kit();
    user_kit.delete_user(request.user_id.clone()).await.unwrap();
    HttpResponse::Ok().json(
        {}
    )
}