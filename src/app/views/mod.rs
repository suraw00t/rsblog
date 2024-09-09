use actix_web::web;

mod root;
mod users;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(users::config).configure(root::config);
}
