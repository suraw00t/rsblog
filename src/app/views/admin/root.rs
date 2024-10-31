use actix_web::{error, web, Error, Responder, Result};

// #[get("/")]
async fn index(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    let html = tmpl
        .render("admin/index.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(web::Html::new(html))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").name("admin::index").to(index))
        .service(web::resource("").to(index));
}
