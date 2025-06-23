use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use serde::Serialize;
use utoipa::openapi::security::{
    AuthorizationCode, Flow, Http, Implicit, OAuth2, Password, Scopes,
};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityRequirement, SecurityScheme};
use utoipa::{openapi, Modify, OpenApi};
use utoipa::{openapi::Server, openapi::ServerBuilder};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable as RedocServable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::{Config, SwaggerUi};

use crate::common::config::Config as ConfigEnv;

pub mod core;
mod models;
pub mod repositories;
mod routes;
pub mod schemas;

#[derive(Debug, Serialize)]
struct OAuth2PasswordBearer;

impl Modify for OAuth2PasswordBearer {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "OAuth2PasswordBearer",
                SecurityScheme::OAuth2(OAuth2::new([Flow::Password(Password::with_refresh_url(
                    ConfigEnv::get_prefix() + "/v1/oauth/login",
                    Scopes::default(),
                    ConfigEnv::get_prefix() + "/v1/oauth/refresh",
                ))])),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    nest(
       (path = "/api", api = routes::Api)
    ),
    modifiers(&OAuth2PasswordBearer),
    security(
        ("OAuth2PasswordBearer" = [])
    )
)]
struct ApiDoc;

fn get_api_path(base: &str) -> String {
    let prefix = ConfigEnv::get_prefix();
    format!("{}{}", prefix.trim_end_matches('/'), base)
}

fn get_servers(base: &str) -> Vec<Server> {
    vec![ServerBuilder::new()
        .url(get_api_path(base))
        .description(Some("Production Server"))
        .build()]
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let openapi_json = "/api-docs/openapi.json";
    let openapi_json_with_prefix = get_api_path(openapi_json);
    let mut doc = ApiDoc::openapi();
    doc.servers = Some(get_servers("/"));

    cfg.service(
        web::scope("/api").configure(routes::config), // .wrap(HttpAuthentication::with_fn(
                                                      //     core::security::Bearer::validator,
                                                      // )),
    )
    .service(
        SwaggerUi::new("/swagger-ui/{_:.*}")
            .url(openapi_json, doc.clone())
            .config(
                Config::new([openapi_json_with_prefix.clone()]), // .validator_url("none")
            ),
    )
    .service(RapiDoc::new(openapi_json_with_prefix).path("/rapidoc"))
    .service(Scalar::with_url("/scalar", doc.clone()))
    .service(Redoc::with_url("/redoc", doc));
}
