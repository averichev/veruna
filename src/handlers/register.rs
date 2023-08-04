use std::fmt::Display;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use actix_web_validator::{Json};
use serde::Deserialize;
use validator::{Validate};
use crate::AppState;
use crate::errors::InternalServerError;

#[derive(Deserialize, Validate)]
pub(crate) struct RegisterForm {
    #[validate(length(min = 3, message = "Должно содержать минимум 3 символа"))]
    username: String,
    #[validate(length(min = 3, message = "Должно содержать минимум 3 символа"))]
    #[validate(must_match = "password_repeat")]
    password: String,
    #[validate(length(min = 3, message = "Должно содержать минимум 3 символа"))]
    #[validate(must_match(other = "password"))]
    password_repeat: String,
}

pub(crate) async fn register_action(form: Json<RegisterForm>, app: Data<AppState>) -> impl Responder {
    let users_repository = &app.repositories.users(); // Create a named variable
    let mut repository = users_repository.lock().await; // Lock the repository
    let user_id_option = repository
        .find_user_id_by_username(form.username.clone())
        .await;
    let mut kit = app.domain.user_kit();
    match user_id_option {
        None => {
            println!("user_id_option none");
            let register_result = kit.register_user(
                form.username.clone(),
                form.password.clone(),
            ).await;
            match register_result {
                Ok(user_id) => {
                    HttpResponse::Ok().json(user_id)
                }
                Err(data_error) => {
                    HttpResponse::InternalServerError().json(
                        InternalServerError { message: data_error.message() }
                    )
                }
            }
        }
        Some(user_id) => {
            println!("user_id_option some");
            HttpResponse::Conflict().json(user_id)
        }
    }
}