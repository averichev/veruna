use std::rc::Rc;
use std::sync::{Arc, Mutex};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpResponse};
use actix_web::body::EitherBody;
use actix_web::http::header::{HeaderValue, WWW_AUTHENTICATE};
use futures_util::future::{LocalBoxFuture, ready, Ready};
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::VerifyWithKey;
use linq::iter::Enumerable;
use sha2::Sha256;
use crate::models::{Claims, CurrentUserTrait};

pub(crate) struct AdminApi {
    current_user: Arc<Mutex<dyn CurrentUserTrait>>,
}

impl AdminApi {
    pub(crate) fn new(current_user: Arc<Mutex<dyn CurrentUserTrait>>) -> AdminApi {
        AdminApi { current_user }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AdminApi
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AdminApiMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminApiMiddleware {
            next_service: Rc::new(service),
            current_user: self.current_user.clone(),
        }))
    }
}

pub struct AdminApiMiddleware<S> {
    next_service: Rc<S>,
    current_user: Arc<Mutex<dyn CurrentUserTrait>>,
}

impl<S, B> AdminApiMiddleware<S>

    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: 'static,
{

    fn unauthorized_response(&self, svc: Rc<S>, req: ServiceRequest) -> LocalBoxFuture<'static, Result<ServiceResponse<EitherBody<B>>, Error>> {
        Box::pin(async move {
            let mut res = svc.call(req).await?;
            let request = res.request().clone();
            let resp = HttpResponse::Unauthorized().finish();
            let new_res = ServiceResponse::new(request, resp)
                .map_into_right_body();
            Ok(new_res)
        })
    }
}

impl<S, B> Service<ServiceRequest> for AdminApiMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(next_service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let header = req.headers().iter()
            .where_by(|(header_name, _)| header_name.to_string().eq("authorization"))
            .select(|(name, value)| Header {
                name: name.to_string(),
                value: value.to_str().unwrap().to_string(),
            })
            .first();
        let svc = self.next_service.clone();
        let mut current_user = self.current_user.clone();
        return match header {
            None => {
                println!("нет заголовка");
                self.unauthorized_response(svc, req)
            }
            Some(h) => {
                println!("заголовок есть - надо провести проверку");
                let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
                let token_str = h.value;
                let verification: Result<Claims, jwt::Error> = token_str.verify_with_key(&key);
                match verification {
                    Ok(n) => {
                        println!("все хорошо {}, {}", n.username, token_str.clone());
                        current_user.lock().unwrap().set_user_name(n.username);
                        Box::pin(async move {
                            let mut res = svc.call(req).await?;
                            res
                                .headers_mut()
                                .insert(
                                    WWW_AUTHENTICATE,
                                    HeaderValue::from_static("Basic realm=\"Restricted\""),
                                );
                            Ok(res.map_into_left_body())
                        })
                    }
                    Err(e) => {
                        println!("неверные данные");
                        println!("{}", e.to_string());
                        self.unauthorized_response(svc, req)
                    }
                }
            }
        }
    }
}


#[derive(Clone)]
struct Header {
    name: String,
    value: String,
}