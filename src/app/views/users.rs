use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/info")]
async fn get_info() -> impl Responder {
    HttpResponse::Ok().body("User Info")
}

#[get("/reset_password")]
async fn reset_password() -> impl Responder {
    HttpResponse::Ok().body("Reset Password")
}

#[get("/profile")]
async fn get_profile() -> impl Responder {
    HttpResponse::Ok().body("User Profile")
}

#[post("/profile")]
async fn set_profile() -> impl Responder {
    HttpResponse::Ok().body("Set Profile")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(get_info)
            .service(reset_password)
            .service(get_profile)
            .service(set_profile),
    );
}
