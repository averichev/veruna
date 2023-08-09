use std::sync::{Arc, Mutex};
use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use hmac::digest::KeyInit;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::AppState;
use crate::models::CurrentUserTrait;

#[derive(Deserialize, Validate)]
pub(crate) struct CurrentUserRequest {
    #[validate(length(min = 3, message = "Должно содержать минимум 3 символа"))]
    jwt: String,
}

#[derive(Serialize)]
pub(crate) struct CurrentUserResponse {
    username: String,
}

pub(crate) async fn handle_form_data(app: Data<AppState>, current_user: Data<Arc<Mutex<dyn CurrentUserTrait>>>) -> impl Responder {
    HttpResponse::Ok().json(
        CurrentUserResponse { username: current_user.lock().unwrap().username() }
    )
}