use actix_web::{get, post, put, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use mongodb::bson::doc;
use page_hunter::{bind_records, paginate_records};
use utoipa::OpenApi;

use crate::api::core::error_handlers;
use crate::api::models::user::{CreateUser, FindUser, UpdateUser, User};
use crate::api::repositories;
use crate::api::schemas::user::{UserBook, UserPage};
use crate::api::schemas::{BindingParams, PaginationParams};

#[derive(OpenApi)]
#[openapi(
    paths(create_user, get_users, get_user, update_user),
    components(schemas(User, UserBook, UserPage))
)]
pub struct UserApi;

#[utoipa::path(
params(
        PaginationParams,
        FindUser,
    ),
    responses(
        (status = 200, description = "List of users", body = UserPage),
        (status = 422, description = "Unprocessable Entity", body = error_handlers::ErrorResponse),
    )
)]
#[get("")]
pub async fn get_users(
    params: web::Query<PaginationParams>,
    find_user: web::Query<FindUser>,
    auth: BearerAuth,
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
        (status = 200, description = "A user", body = User),
        (status = 404, description = "User not found", body = error_handlers::ErrorResponse),
        (status = 422, description = "Unprocessable Entity", body = error_handlers::ErrorResponse),
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
        (status = 201, description = "User created successfully", body = User),
        (status = 422, description = "Unprocessable Entity", body = error_handlers::ErrorResponse),
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
        (status = 200, description = "User updated successfully", body = User),
        (status = 422, description = "Unprocessable Entity", body = error_handlers::ErrorResponse),
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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(get_user)
            .service(get_users)
            .service(create_user)
            .service(update_user),
    );
}
