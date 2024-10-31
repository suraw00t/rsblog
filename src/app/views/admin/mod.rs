use actix_web::web;

mod profile;
mod root;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .configure(root::config)
            .configure(profile::config),
    );
}
