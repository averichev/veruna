use actix_web::{HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct FormData {
    login: String,
    password: String,
}
pub(crate) async fn handle_form_data(form: web::Form<FormData>) -> HttpResponse {
    let field1_value = &form.login;
    let field2_value = &form.password;
    HttpResponse::Ok().body(format!("Received form data: {} - {}", field1_value, field2_value))
}