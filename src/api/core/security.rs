#![allow(unused)]

use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::{basic::BasicAuth, bearer::BearerAuth};

pub struct Bearer;
impl Bearer {
    pub async fn validator(
        req: ServiceRequest,
        credentials: BasicAuth,
    ) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
        eprintln!("{credentials:?}");

        if credentials.user_id().contains('x') {
            return Err((actix_web::error::ErrorBadRequest("user ID contains x"), req));
        }

        Ok(req)
    }
}

pub struct Basic;
impl Basic {
    pub async fn validator(
        req: ServiceRequest,
        credentials: Option<BearerAuth>,
    ) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
        // let credentials else {
        //     return Err((actix_web::error::ErrorBadRequest("no bearer header"), req));
        // };

        eprintln!("Bearer validate >>>> {credentials:?}");
        match credentials {
            Some(_) => return Ok(req),
            None => return Err((actix_web::error::ErrorUnauthorized("no bearer header"), req)),
        };

        // if credentials.token().contains('x') {
        //     return Err((actix_web::error::ErrorBadRequest("token contains x"), req));
        // }
    }
}
