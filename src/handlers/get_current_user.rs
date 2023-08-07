use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use actix_web_validator::Json;
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::VerifyWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use validator::Validate;
use veruna_domain::users::models::claims::ClaimsTrait;
use crate::AppState;
use crate::models::Claims;

#[derive(Deserialize, Validate)]
pub(crate) struct CurrentUserRequest {
    #[validate(length(min = 3, message = "Должно содержать минимум 3 символа"))]
    jwt: String,
}

#[derive(Serialize)]
pub(crate) struct CurrentUserResponse {
    username: String,
}

pub(crate) async fn handle_form_data(request: Json<CurrentUserRequest>, app: Data<AppState>) -> impl Responder {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let token_str = request.jwt.clone();
    let claims: Claims = token_str.verify_with_key(&key).unwrap();
    HttpResponse::Ok().json(
        CurrentUserResponse { username: claims.username }
    )
}