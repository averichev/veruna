use std::fmt::Display;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use actix_web_validator::{Json};
use serde::Deserialize;
use validator::{Validate};
use veruna_domain::users::LoginUser;
use crate::AppState;
use crate::response::LoginResponse;

#[derive(Deserialize, Validate)]
pub(crate) struct FormData {
    #[validate(length(min = 3, message = "Должно содержать минимум 3 символа"))]
    username: String,
    #[validate(length(min = 3, message = "Должно содержать минимум 3 символа"))]
    password: String,
}

pub(crate) async fn handle_form_data(form: Json<FormData>, app: Data<AppState>) -> impl Responder {
    let user_kit = app.domain.user_kit();
    let verify = user_kit.verify_user_password(LoginUser {
        username: form.username.clone(),
        password: form.password.clone(),
    }
    ).await;
    match verify {
        Ok(result) => {
            match result {
                true => {
                    HttpResponse::Ok().json(LoginResponse { result, user: None }) // возвращать юзера
                }
                false => {
                    println!("Не пройдена проверка");
                    HttpResponse::Unauthorized().json(LoginResponse { result: false, user: None })
                }
            }
        }
        Err(_) => {
            println!("Пользователь не найден");
            HttpResponse::Unauthorized().json(LoginResponse { result: false, user: None })
        }
    }
}