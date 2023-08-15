mod models;

use std::sync::{Arc, Mutex};
use actix_web::{HttpResponse, Responder, web};
use actix_web::web::Data;
use crate::AppState;
use crate::handlers::user::models::{DeleteUserRequest, UserList};
use crate::models::CurrentUserTrait;

pub(crate) async fn list(app: Data<AppState>) -> impl Responder {
    let user_kit = app.domain.user_kit();
    let list = user_kit.get_user_list().await.unwrap();
    let result = UserList::new(list);
    HttpResponse::Ok().json(
        result
    )
}

pub(crate) async fn delete(app: Data<AppState>,
                           request: web::Path<DeleteUserRequest>,
                           current_user: Data<Arc<Mutex<dyn CurrentUserTrait>>>) -> impl Responder {
    let current_user_id = current_user.lock().unwrap().id();
    if current_user_id == request.user_id {
        return HttpResponse::UnprocessableEntity().json(
            {

            }
        )
    }
    let user_kit = app.domain.user_kit();
    user_kit.delete_user(request.user_id.clone()).await.unwrap();
    HttpResponse::Ok().json(
        {}
    )
}