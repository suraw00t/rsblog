use actix_web::{get, post, web, HttpResponse, Responder};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use serde_json::json;
use utoipa::OpenApi;

use crate::api::core::error_handlers;
use crate::api::models::user::User;

#[derive(OpenApi)]
#[openapi(paths(create_user, get_users, get_user), components(schemas(User)))]
pub struct UserApi;

#[utoipa::path(
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 500, description = "Internal server error", body = error_handlers::ErrorResponse)
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
        (status = 500, description = "Internal server error", body = error_handlers::ErrorResponse)
    )
)]
#[get("/users")]
pub async fn get_users(db: web::Data<Database>) -> impl Responder {
    let collection = db.collection::<User>("users");
    match collection.find(doc! {}).await {
        Ok(cursor) => match cursor.try_collect::<Vec<User>>().await {
            Ok(users) => {
                log::debug!("Found {:?} users", users);
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

#[utoipa::path(
    responses(
        (status = 200, description = "A user", body = User),
        (status = 404, description = "User not found", body = error_handlers::ErrorResponse),
        (status = 422, description = "Invalid ObjectID", body = error_handlers::ErrorResponse),
    )
)]
#[get("/users/{user_id}")]
async fn get_user(
    user_id: web::Path<String>,
    db: web::Data<Database>,
) -> Result<HttpResponse, error_handlers::ApiError> {
    // Simulate user lookup
    let oid = ObjectId::parse_str(user_id.as_str());
    if oid.is_ok() {
        let collection = db.collection::<User>("users");
        match collection
            .find_one(doc! {
               "_id": oid.unwrap()
            })
            .await
        {
            Ok(user) => match user {
                Some(user) => {
                    log::debug!("{:?} {:?}", user, user.id());
                    Ok(HttpResponse::Ok().json(user))
                }
                None => Err(error_handlers::ApiError::NotFound("User".to_string())),
            },
            Err(e) => Err(error_handlers::ApiError::UnprocessableEntity(e.to_string())),
        }
    } else {
        Err(error_handlers::ApiError::UnprocessableEntity(
            "Invalid ObjectID".to_string(),
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
