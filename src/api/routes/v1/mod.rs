use actix_web::web;
use utoipa::OpenApi;

mod auth;
mod root;
mod users;

#[derive(OpenApi)]
#[openapi(nest(
    (path = "/auth", api = auth::AuthApi),
    (path = "/users", api = users::UserApi),
))]
pub struct V1Api;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(root::config)
        .service(web::scope("/users").configure(users::config))
        .service(web::scope("/auth").configure(auth::config));
}
