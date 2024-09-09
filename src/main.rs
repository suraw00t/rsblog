use actix_web::{middleware, web, App, HttpServer};

mod api;
mod app;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let config = api::core::config::Config::from_env();
    let database = db::connect_to_mongodb(&config)
        .await
        .expect("Failed to connect to MongoDB");

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default()) // <- limit size of the payload (global configuration)
            .app_data(web::Data::new(app::initialize_template()))
            .app_data(web::Data::new(database.clone()))
            .configure(api::config)
            .configure(app::config)
    })
    .workers(1)
    .bind((std::net::Ipv4Addr::LOCALHOST, 8081))?
    .run()
    .await
}
