use std::fmt::Display;
use actix_web::{HttpResponse, Responder};
use actix_web_validator::{Json};
use serde::Deserialize;
use validator::{Validate};

#[derive(Deserialize, Validate)]
pub(crate) struct FormData {
    #[validate(length(min = 3, message = "Должно содержать минимум 3 символа"))]
    username: String,
    #[validate(length(min = 3, message = "Должно содержать минимум 3 символа"))]
    password: String,
}

pub(crate) async fn handle_form_data(form: Json<FormData>) -> impl Responder {
    HttpResponse::Ok().body(format!("Received form data: {} - {}",
                                    form.username,
                                    form.password
    ))
}