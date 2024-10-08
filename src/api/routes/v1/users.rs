use crate::api::models::user::User;
use actix_web::{get, post, web, HttpResponse, Responder};
use futures::TryStreamExt;
use mongodb::Database;
use serde_json::json;
use utoipa::OpenApi;

#[path = "../../core/error_handlers.rs"]
mod error_handlers;

#[derive(OpenApi)]
#[openapi(paths(create_user, get_users), components(schemas(User)))]
pub struct UserApi;

#[utoipa::path(
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 500, description = "Internal server error")
    ),
)]
#[post("/users")]
pub async fn create_user(db: web::Data<Database>, user: web::Json<User>) -> impl Responder {
    let mut user_data = user.into_inner();
    let collection = db.collection::<User>("users");
    match collection.insert_one(&user_data).await {
        Ok(result) => {
            if let Some(id) = result.inserted_id.as_object_id() {
                user_data = user_data.with_id(id);
                HttpResponse::Created().json(user_data)
            } else {
                HttpResponse::InternalServerError().json(json!({"error": "Failed to generate ID"}))
            }
        }
        // Err(_) => HttpResponse::InternalServerError().finish(),
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Failed to create user"}))
        }
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "List of users", body = Vec<User>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/users")]
pub async fn get_users(db: web::Data<Database>) -> impl Responder {
    let collection = db.collection::<User>("users");
    match collection.find(mongodb::bson::doc! {}).await {
        Ok(cursor) => match cursor.try_collect::<Vec<User>>().await {
            Ok(users) => {
                println!("Found {:?} users", users);
                HttpResponse::Ok().json(users)
            }
            Err(e) => {
                eprintln!("Error collecting users: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(e) => {
            eprintln!("Error finding users: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/users/{user_id}")]
async fn get_user(user_id: web::Path<String>) -> Result<HttpResponse, error_handlers::ApiError> {
    // Simulate user lookup
    if user_id.as_str() == "123" {
        Ok(HttpResponse::Ok().json(json!({"id": "123", "name": "John Doe"})))
    } else {
        Err(error_handlers::ApiError::NotFound(
            "User not found".to_string(),
        ))
    }
}

#[get("/users/protected")]
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
    cfg.service(protected_resource)
        .service(get_user)
        .service(get_users)
        .service(create_user);
}
