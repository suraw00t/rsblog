use actix_web::{
    dev::{ResourceMap, Service},
    middleware,
    test::TestRequest,
    web, App, HttpServer,
};
use std::{borrow::Borrow, cell::RefCell, collections::HashMap, io};
use utoipa_actix_web::AppExt;

mod api;
mod app;
mod common;

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
        let prefix = common::config::Config::get_prefix();
        let fake_req = TestRequest::default().to_http_request();
        let url = routes
            .url_for(&fake_req, name, elements)
            .or(Err(tera::Error::msg("resource not found")))?;
        let full_path = format!("{}{}", prefix, url.path());
        Ok(tera::Value::String(full_path))
    })
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    common::config::Config::init_from_env();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    common::db::init_db().await;

    log::info!("starting HTTP server");
    let mut tera = app::initialize_template();
    tera.register_function("url_for", tera_url_for);

    HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .into_app()
            // .split_for_parts()
            .configure(api::config)
            .configure(app::config)
            .app_data(web::JsonConfig::default())
            .app_data(web::Data::new(tera.clone()))
            .wrap(common::forwarded_prefix::ForwardPrefix)
            .wrap(middleware::Logger::default())
            .wrap_fn(move |req, srv| {
                ROUTES_KEY.with(|routes| {
                    routes
                        .borrow_mut()
                        .get_or_insert_with(|| req.resource_map().clone());
                });
                srv.borrow().call(req)
            })
    })
    .workers(common::config::Config::get_workers())
    .bind_auto_h2c((
        common::config::Config::get_address(),
        common::config::Config::get_port(),
    ))?
    .run()
    .await
}
