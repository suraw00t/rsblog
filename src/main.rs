use actix_web::dev::Service;
use actix_web::{dev::ResourceMap, middleware, test::TestRequest, web, App, HttpServer};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;

mod api;
mod app;
mod db;

thread_local! {
    static ROUTES_KEY: RefCell<Option<ResourceMap>> = RefCell::new(None);
}

fn tera_url_for(args: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let name = args["name"]
        .as_str()
        .ok_or(tera::Error::msg("`name` should be a string"))?;
    let empty_elements = tera::Value::Array(vec![]);
    let elements_iter = args
        .get("elements")
        .unwrap_or(&empty_elements)
        .as_array()
        .ok_or(tera::Error::msg("`elements` should be an array"))?
        .iter();
    let mut elements = vec![];
    for elem in elements_iter {
        elements.push(elem.as_str().ok_or(tera::Error::msg(
            "`elements` array should contain only strings",
        ))?);
    }
    ROUTES_KEY.with(|routes| {
        let routes_ref = routes.borrow();
        let routes = routes_ref.as_ref().ok_or(tera::Error::msg(
            "`url_for` should only be called in request context",
        ))?;
        let fake_req = TestRequest::default().to_http_request();
        let url = routes
            .url_for(&fake_req, name, elements)
            .or(Err(tera::Error::msg("resource not found")))?;
        Ok(tera::Value::String(url.path().to_string()))
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let config = api::core::config::Config::from_env();
    let database = db::connect_to_mongodb(&config)
        .await
        .expect("Failed to connect to MongoDB");

    log::info!("starting HTTP server at http://localhost:8080");
    let mut tera = app::initialize_template();
    tera.register_function("url_for", tera_url_for);

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default()) // <- limit size of the payload (global configuration)
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(database.clone()))
            .configure(api::config)
            .configure(app::config)
            .wrap_fn(move |req, srv| {
                ROUTES_KEY.with(|routes| {
                    routes
                        .borrow_mut()
                        .get_or_insert_with(|| req.resource_map().clone());
                });
                srv.borrow().call(req)
            })
    })
    .workers(
        std::env::var("WORKERS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1),
    )
    .bind((
        std::env::var("ADDR6")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(std::net::Ipv6Addr::LOCALHOST),
        std::env::var("PORT6")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8081),
    ))?
    .bind((
        std::env::var("ADDR")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(std::net::Ipv4Addr::LOCALHOST),
        std::env::var("PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080),
    ))?
    .run()
    .await
}
