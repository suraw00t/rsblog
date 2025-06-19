use actix_web::web;

use utoipa::{openapi::Server, openapi::ServerBuilder, OpenApi};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::{Config, SwaggerUi};

pub mod core;
mod models;
pub mod repositories;
mod routes;
pub mod schemas;

#[derive(OpenApi)]
#[openapi(
    nest(
       (path = "/api", api = routes::v1::V1Api)
    ),
)]
struct ApiDoc;

fn get_servers(base: &str) -> Vec<Server> {
    match std::env::var("PREFIX") {
        Ok(prefix) => vec![ServerBuilder::new()
            .url(&format!("{}", prefix.trim_end_matches('/')))
            .description(Some("Production Server"))
            .build()],
        Err(_) => vec![ServerBuilder::new()
            .url(base)
            .description(Some("Production Server"))
            .build()],
    }
}

fn get_api_path(base: &str) -> String {
    match std::env::var("PREFIX") {
        Ok(prefix) => format!("{}{}", prefix.trim_end_matches('/'), base),
        Err(_) => base.to_string(),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let openapi_json = "/api-docs/openapi.json";
    let openapi_json_with_prefix = get_api_path(openapi_json);
    let mut doc = ApiDoc::openapi();
    doc.servers = Some(get_servers("/"));

    cfg.service(web::scope("/api").configure(routes::config))
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url(openapi_json, doc.clone())
                .config(Config::new([openapi_json_with_prefix.clone()]).validator_url("")),
        )
        .service(RapiDoc::new(openapi_json_with_prefix.clone()).path("/rapidoc"))
        .service(Scalar::with_url("/scalar", doc.clone()))
        .service(Redoc::with_url("/redoc", doc.clone()));
}
