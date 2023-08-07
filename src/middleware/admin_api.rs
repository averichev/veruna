use std::collections::BTreeMap;
use actix_web::Error;
use std::future::{ready, Ready};

use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform}, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::VerifyWithKey;
use linq::iter::Enumerable;
use sha2::Sha256;

pub struct AdminApi;

impl<S, B> Transform<S, ServiceRequest> for AdminApi
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AdminApiMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminApiMiddleware { service }))
    }
}

pub struct AdminApiMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AdminApiMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, service_request: ServiceRequest) -> Self::Future {
        println!("admin_api, {}", service_request.path());
        let header = service_request.headers().iter()
            .where_by(|(header_name, _)| header_name.to_string().eq("authorization"))
            .select(|(name, value)| Header {
                name: name.to_string(),
                value: value.to_str().unwrap().to_string(),
            })
            .first();
        let (request, _pl) = service_request.into_parts();

        let mut response;
        match header {
            None => {
                println!("нет заголовка");
                response = HttpResponse::Unauthorized()
                    .finish()
                    .map_into_right_body();
            }
            Some(h) => {
                println!("заголовок есть - надо провести проверку");
                let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
                let token_str = h.value;
                let verification: Result<BTreeMap<String, String>, jwt::Error> = token_str.verify_with_key(&key);
                match verification {
                    Ok(n) => {
                        println!("все хорошо {}, {}", n["sub"], token_str.clone());
                        response = HttpResponse::Ok()
                            .finish()
                            .map_into_right_body();
                        return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
                    },
                    Err(e) => {
                        println!("неверные данные");
                        println!("{}", e.to_string());
                        response = HttpResponse::Unauthorized()
                            .finish()
                            .map_into_right_body();
                    }
                }
                response = HttpResponse::Unauthorized()
                    .finish()
                    .map_into_right_body();
            }
        }
        return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
    }
}

#[derive(Clone)]
struct Header {
    name: String,
    value: String,
}