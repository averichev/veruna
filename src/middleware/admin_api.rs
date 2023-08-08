use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use actix_web::http::header::{HeaderValue, WWW_AUTHENTICATE};
use futures_util::future::{LocalBoxFuture, ready, Ready};

pub struct AdminApi;

impl<S, B> Transform<S, ServiceRequest> for AdminApi
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
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
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res: ServiceResponse<B> = fut.await?;
            res
                .headers_mut()
                .insert(
                    WWW_AUTHENTICATE,
                    HeaderValue::from_static("Basic realm=\"Restricted\""),
                );
            println!("Hi from response");
            Ok(res)
        })
    }
}