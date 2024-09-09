use actix_web::web;

pub mod core;
mod models;
mod routes;

use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    nest(
       (path = "/api", api = routes::v1::V1Api)
    )
)]
struct ApiDoc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(routes::config))
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .service(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .service(Redoc::with_url("/redoc", ApiDoc::openapi()));
}
