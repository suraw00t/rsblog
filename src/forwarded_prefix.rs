use std::future::{ready, Ready};

use actix_web::http::header::HeaderValue;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http, Error,
};
use actix_web_lab::header::X_FORWARDED_PREFIX;
use futures_util::future::LocalBoxFuture;

pub struct ForwardPrefix;

impl Default for ForwardPrefix {
    fn default() -> Self {
        Self
    }
}

impl<S, B> Transform<S, ServiceRequest> for ForwardPrefix
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut prefix = "".to_string();
        match req.headers().get(X_FORWARDED_PREFIX) {
            None => {}
            Some(header) => {
                log::debug!(
                    "X_FORWARDED_PREFIX: {:?}",
                    req.headers().get(X_FORWARDED_PREFIX)
                );
                prefix = header.to_str().unwrap().to_string();
            }
        }
        let path = (&req).path().to_string();
        let location = if prefix.len() == 0 {
            path
        } else {
            prefix + path.as_str()
        };

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            res.headers_mut().insert(
                http::header::LOCATION,
                HeaderValue::from_str(location.as_str()).unwrap(),
            );

            Ok(res)
        })
    }
}
