use actix_web::web;

use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

pub mod core;
mod models;
mod routes;

#[derive(OpenApi)]
#[openapi(
    nest(
       (path = get_api_path(), api = routes::v1::V1Api)
    ),
    modifiers()
)]
struct ApiDoc;

fn get_api_path() -> String {
    match std::env::var("PREFIX") {
        Ok(prefix) => format!("{}/api", prefix.trim_end_matches('/')),
        Err(_) => "/api".to_string(),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let openapi_json = "/api-docs/openapi.json";
    cfg.service(web::scope("/api").configure(routes::config))
        .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(openapi_json, ApiDoc::openapi()))
        .service(RapiDoc::new(openapi_json).path("/rapidoc"))
        .service(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .service(Redoc::with_url("/redoc", ApiDoc::openapi()));
}
