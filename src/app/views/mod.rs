use actix_web::web;

mod admin;
mod root;
mod users;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(admin::config)
        .configure(users::config)
        .configure(root::config);
}
