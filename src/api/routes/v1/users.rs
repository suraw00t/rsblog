use actix_web::{get, post, web, HttpResponse, Responder};
use mongodb::bson::doc;
use page_hunter::{bind_records, paginate_records};
use serde_json::json;
use utoipa::OpenApi;

use crate::api::core::error_handlers;
use crate::api::models::user::{CreateUser, User};
use crate::api::repositories;
use crate::api::schemas::user::{UserBook, UserPage};
use crate::api::schemas::{BindingParams, PaginationParams};

#[derive(OpenApi)]
#[openapi(
    paths(create_user, get_users, get_user),
    components(schemas(User, UserBook, UserPage))
)]
pub struct UserApi;

#[utoipa::path(
params(
        PaginationParams,
    ),
    responses(
        (status = 200, description = "List of users", body = UserPage),
        (status = 500, description = "Internal server error", body = error_handlers::ErrorResponse)
    )
)]
#[get("")]
pub async fn get_users(params: web::Query<PaginationParams>) -> impl Responder {
    let user_repo = repositories::UserRepository::new().await;
    let users = user_repo.get(None).await;
    match users {
        Ok(Some(users)) => {
            let page: UserPage =
        match paginate_records(&users, params.get_page(), params.get_size()) {
            Ok(page) => page,
            Err(e) => return Err(error_handlers::ApiError::UnprocessableEntity(e.to_string())),
        };
            Ok(HttpResponse::Ok().json(page))
        },
        Ok(None) => Err(error_handlers::ApiError::NotFound("User".to_string())),
        Err(e) => Err(error_handlers::ApiError::UnprocessableEntity(e.to_string())),
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "A user", body = User),
        (status = 404, description = "User not found", body = error_handlers::ErrorResponse),
        (status = 422, description = "Invalid ObjectID", body = error_handlers::ErrorResponse),
    )
)]
#[get("/{user_id}")]
async fn get_user(
    user_id: web::Path<String>,
    // db: web::Data<Database>,
) -> Result<HttpResponse, error_handlers::ApiError> {
    let user_repo = repositories::UserRepository::new().await;
    let user = user_repo.get_by_id(user_id.to_string()).await;
    match user {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Err(error_handlers::ApiError::NotFound("User".to_string())),
        Err(_) => Err(error_handlers::ApiError::UnprocessableEntity(
            "Invalid ObjectID".to_string(),
        )),
    }
}
#[utoipa::path(
    
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 500, description = "Internal server error", body = error_handlers::ErrorResponse)
    ),
)]
#[post("")]
pub async fn create_user(
    // db: web::Data<Database>,
    user_data: web::Json<CreateUser>,
) -> impl Responder {
    let user_repo = repositories::UserRepository::new().await;
    let user = user_repo.create(user_data.into_inner()).await;
    match user {
        Ok(Some(user)) => Ok(HttpResponse::Created().json(user)),
        Ok(None) => Err(error_handlers::ApiError::NotFound("User".to_string())),
        Err(e) => Err(error_handlers::ApiError::UnprocessableEntity(e.to_string())),
    }
}

#[get("/protected")]
async fn protected_resource() -> Result<HttpResponse, error_handlers::ApiError> {
    // Simulate authorization check
    if 1 > 2 {
        Ok(HttpResponse::Ok().json(json!({"message": "Access granted"})))
    } else {
        Err(error_handlers::ApiError::Forbidden(
            "Access denied".to_string(),
        ))
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(protected_resource)
            .service(get_user)
            .service(get_users)
            .service(create_user),
    );
}
