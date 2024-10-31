use actix_web::{error, web, Error, Responder, Result};

// #[get("/")]
async fn profile(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    let html = tmpl
        .render("admin/profile.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(web::Html::new(html))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/profile").name("admin::profile").to(profile));
}
