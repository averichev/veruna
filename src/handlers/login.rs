use std::fmt::Display;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use actix_web_validator::{Json};
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::SignWithKey;
use serde::Deserialize;
use sha2::Sha256;
use validator::{Validate};
use veruna_domain::users::LoginUser;
use crate::AppState;
use crate::models::Claims;
use crate::response::{LoggedUser, LoginResponse, LoginResponseData};

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
            let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
            let claims = Claims { username: result.username(), id: result.id() };
            let token_str = claims.sign_with_key(&key).unwrap();
            HttpResponse::Ok().json(
                LoginResponse {
                    result: true,
                    data: Some(LoginResponseData {
                        user: LoggedUser {
                            username: result.username()
                        },
                        token: token_str,
                    }),
                }
            )
        }
        Err(_) => {
            println!("Пользователь не найден");
            HttpResponse::Unauthorized().json(LoginResponse { result: false, data: None })
        }
    }
}