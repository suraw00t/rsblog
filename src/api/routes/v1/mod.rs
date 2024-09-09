use actix_web::web;
use utoipa::OpenApi;

mod root;
pub mod users;

#[derive(OpenApi)]
#[openapi(nest((path = "/v1", api = users::UserApi)))]
pub struct V1Api;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .configure(root::config)
            .configure(users::config),
    );
}
