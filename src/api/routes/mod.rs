use actix_web::web;
use utoipa::OpenApi;

pub mod v1;

#[derive(OpenApi)]
#[openapi(nest((path = "/v1", api = v1::V1Api)))]
pub struct Api;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1").configure(v1::config));
}
