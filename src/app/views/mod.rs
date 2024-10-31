use actix_web::web;

mod account;
mod admin;
mod root;
mod users;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(admin::config)
        .configure(users::config)
        .configure(root::config)
        .configure(account::config);
}
