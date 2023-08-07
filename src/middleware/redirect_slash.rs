use std::future::{ready, Ready};

use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http, Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use regex::Regex;

pub struct RedirectSlash;

impl<S, B> Transform<S, ServiceRequest> for RedirectSlash
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = RedirectSlashMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RedirectSlashMiddleware { service }))
    }
}

pub struct RedirectSlashMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RedirectSlashMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        // Change this to see the change in outcome in the browser.
        // Usually this boolean would be acquired from a password check or other auth verification.
        let is_logged_in = false;

        let uri = request.uri().clone().to_string();
        let re = Regex::new("(\\w+)\\.(\\w+)$").unwrap();
        if !re.is_match(uri.as_str()) && !Regex::new("/$").unwrap().is_match(uri.as_str()) {
            let (request, _pl) = request.into_parts();
            let mut redirect = uri.to_string();
            redirect.push('/');
            let response = HttpResponse::TemporaryRedirect()
                .insert_header((http::header::LOCATION, redirect))
                .finish()
                // constructed responses map to "right" body
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let res = self.service.call(request);

        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}