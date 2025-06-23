use actix_web::{get, post, put, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use mongodb::bson::doc;
use page_hunter::{bind_records, paginate_records};
use utoipa::{OpenApi, openapi::security::SecurityScheme};
use utoipa::openapi::security::{HttpBuilder, HttpAuthScheme};
use actix_multipart::form::MultipartForm;

use crate::api::core::error_handlers;
use crate::api::models::users::{CreateUser, FindUser, UpdateUser, User};
use crate::api::repositories;
use crate::api::schemas::users::{UserBook, UserPage, PictureProfile};
use crate::api::schemas::{BindingParams, PaginationParams};

#[derive(OpenApi)]
#[openapi(
    paths(create_user, get_users, get_user, update_user, picture_profile),
    components(schemas(User, UserBook, UserPage, PictureProfile))
)]
pub struct UserApi;

#[utoipa::path(
    params(
        PaginationParams,
        FindUser,
    ),
    responses(
        (status = OK, description = "List of users", body = UserPage),
        (status = UNPROCESSABLE_ENTITY, description = "Unprocessable Entity", body = error_handlers::ErrorResponse),
    ),
    security(),
    
)]
#[get("")]
pub async fn get_users(
    params: web::Query<PaginationParams>,
    find_user: web::Query<FindUser>,
    // _auth: BearerAuth,
) -> impl Responder {
    let user_repo = repositories::UserRepository::new().await;
    let users = user_repo.get(Some(find_user.into_inner())).await;
    match users {
        Ok(Some(users)) => {
            let page: UserPage =
                match paginate_records(&users, params.get_page(), params.get_size()) {
                    Ok(page) => page,
                    Err(e) => {
                        return Err(error_handlers::ApiError::UnprocessableEntity(e.to_string()))
                    }
                };
            Ok(HttpResponse::Ok().json(page))
        }
        Ok(None) => Err(error_handlers::ApiError::NotFound("User".to_string())),
        Err(e) => Err(error_handlers::ApiError::UnprocessableEntity(e.to_string())),
    }
}

#[utoipa::path(
    responses(
        (status = OK, description = "A user", body = User),
        (status = NOT_FOUND, description = "User not found", body = error_handlers::ErrorResponse),
        (status = UNPROCESSABLE_ENTITY, description = "Unprocessable Entity", body = error_handlers::ErrorResponse),
    )
)]
#[get("/{user_id}")]
async fn get_user(user_id: web::Path<String>) -> Result<HttpResponse, error_handlers::ApiError> {
    let user_repo = repositories::UserRepository::new().await;
    let user = user_repo.get_by_id(user_id.to_string()).await;
    match user {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Err(error_handlers::ApiError::NotFound("User".to_string())),
        Err(e) => Err(error_handlers::ApiError::UnprocessableEntity(e.to_string())),
    }
}

#[utoipa::path(
    responses(
        (status = CREATED, description = "User created successfully", body = User),
        (status = UNPROCESSABLE_ENTITY, description = "Unprocessable Entity", body = error_handlers::ErrorResponse),
    ),
)]
#[post("")]
pub async fn create_user(user_data: web::Json<CreateUser>) -> impl Responder {
    let user_repo = repositories::UserRepository::new().await;
    let user = user_repo.create(user_data.into_inner()).await;
    match user {
        Ok(Some(user)) => Ok(HttpResponse::Created().json(user)),
        Ok(None) => Err(error_handlers::ApiError::NotFound("User".to_string())),
        Err(e) => Err(error_handlers::ApiError::UnprocessableEntity(e.to_string())),
    }
}

#[utoipa::path(
    params(
        ("user_id" = String, Path, description = "User id"),
    ),
    responses(
        (status = OK, description = "User updated successfully", body = User),
        (status = UNPROCESSABLE_ENTITY, description = "Unprocessable Entity", body = error_handlers::ErrorResponse),
    ),
)]
#[put("/{user_id}")]
pub async fn update_user(
    user_id: web::Path<String>,
    user_data: web::Json<UpdateUser>,
) -> impl Responder {
    let user_repo = repositories::UserRepository::new().await;
    let user = user_repo
        .update(user_id.to_string(), user_data.into_inner())
        .await;
    match user {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Err(error_handlers::ApiError::NotFound("User".to_string())),
        Err(e) => Err(error_handlers::ApiError::UnprocessableEntity(e.to_string())),
    }
}

#[utoipa::path(
    request_body(content = PictureProfile, content_type = "multipart/form-data")
)]
#[post("/{user_id}/picture")]
async fn picture_profile(user_id: web::Path<String>, MultipartForm(form): MultipartForm<PictureProfile>) -> impl Responder {
    log::debug!("User ID: {}", user_id);
    // let name = form.name.to_string();
    let name = form.name.as_ref().map_or("No name provided".to_string(), |n| n.to_string());
    let file = &form.file;
    format!(
        "Greetings: name: {name}, type: {} size: {} file_name: {}!",
        file.content_type
            .as_ref()
            .map(|mime| mime.to_string())
            .unwrap_or_default(),
        file.size,
        file.file_name.as_ref().unwrap_or(&String::new())
    )
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user)
        .service(get_users)
        .service(create_user)
        .service(update_user)
        .service(picture_profile);
}
