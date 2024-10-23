use actix_web::web;
use utoipa::OpenApi;

mod root;
pub mod users;

#[derive(OpenApi)]
#[openapi(nest((path = "/v1", api = users::UserApi)))]
pub struct V1Api;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(root::config)
        .service(web::scope("/v1").configure(users::config));
}
