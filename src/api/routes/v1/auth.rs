use actix_web::{get, post, put, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use mongodb::bson::doc;
use page_hunter::{bind_records, paginate_records};
use utoipa::{OpenApi, openapi::security::SecurityScheme};
use utoipa::openapi::security::{HttpBuilder, HttpAuthScheme};
use actix_multipart::form::MultipartForm;

use crate::api::core::error_handlers;
use crate::api::models::users::{CreateUser, FindUser, UpdateUser, User};
use crate::api::repositories;
use crate::api::schemas::users::{UserBook, UserPage, PictureProfile, UploadPictureProfile};
use crate::api::schemas::{BindingParams, PaginationParams};

#[derive(OpenApi)]
#[openapi(
    paths(login, refresh_token),
    // components(schemas(User, UserBook, UserPage, PictureProfile, UploadPictureProfile)),
)]
pub struct AuthApi;

#[utoipa::path(
    responses(
        (status = OK, description = "Ok"),
        (status = UNAUTHORIZED, description = "Unauthorized"),
        (status = UNPROCESSABLE_ENTITY, description = "Unprocessable Entity", body = error_handlers::ErrorResponse),
    ),
    security(),
    
)]
#[post("/login")]
pub async fn login(
) -> Result<HttpResponse, error_handlers::ApiError> {
    let response = serde_json::json!({"access_token": "qwertyuio", "refresh_token": "123456789", "expires_in": 10, "token_type": "Bearer"});
    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    responses(
        (status = OK, description = "Ok"),
        (status = UNAUTHORIZED, description = "Unauthorized"),
        (status = UNPROCESSABLE_ENTITY, description = "Unprocessable Entity", body = error_handlers::ErrorResponse),
    ),
    security(),
    
)]
#[post("/refresh")]
pub async fn refresh_token(
) -> Result<HttpResponse, error_handlers::ApiError> {
    let response = serde_json::json!({"access_token": "666666", "refresh_token": "123456789", "expires_in": 10});
    Ok(HttpResponse::Ok().json(response))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(refresh_token);
}

