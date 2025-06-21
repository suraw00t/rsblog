use actix_web::{
    dev::{ResourceMap, Service},
    middleware,
    test::TestRequest,
    web, App, HttpServer,
};
use std::{borrow::Borrow, cell::RefCell, collections::HashMap};

mod api;
mod app;
mod common;

thread_local! {
    static ROUTES_KEY: RefCell<Option<ResourceMap>> = RefCell::new(None);
    static REQUEST_PREFIX: RefCell<Option<String>> = RefCell::new(None);
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
        let prefix = REQUEST_PREFIX.with(|p| p.borrow().clone().unwrap_or_default());
        let fake_req = TestRequest::default().to_http_request();
        let url = routes
            .url_for(&fake_req, name, elements)
            .or(Err(tera::Error::msg("resource not found")))?;
        let path = url.path();
        let full_path = if prefix.is_empty() {
            path.to_string()
        } else {
            format!("{}{}", prefix, path)
        };

        Ok(tera::Value::String(full_path))
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = api::core::config::Config::from_env();
    common::db::init_db(&config).await;

    log::info!("starting HTTP server");
    let mut tera = app::initialize_template();
    tera.register_function("url_for", tera_url_for);

    HttpServer::new(move || {
        App::new()
            .wrap(common::forwarded_prefix::ForwardPrefix)
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default())
            .app_data(web::Data::new(tera.clone()))
            .configure(api::config)
            .configure(app::config)
            .wrap_fn(move |req, srv| {
                ROUTES_KEY.with(|routes| {
                    routes
                        .borrow_mut()
                        .get_or_insert_with(|| req.resource_map().clone());
                });
                if let Ok(prefix) = std::env::var("PREFIX") {
                    REQUEST_PREFIX.with(|p| {
                        *p.borrow_mut() = Some(prefix);
                    });
                }
                srv.borrow().call(req)
            })
    })
    .workers(
        std::env::var("WORKERS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1),
    )
    .bind_auto_h2c((
        std::env::var("ADDR")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(std::net::Ipv4Addr::UNSPECIFIED),
        std::env::var("PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080),
    ))?
    .run()
    .await
}
