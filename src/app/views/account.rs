use actix_web::{error, web, Error, Responder, Result};

async fn signup(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    let html = tmpl
        .render("account/signup.html", &tera::Context::new())
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(web::Html::new(html))
}

async fn signin(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    let html = tmpl
        .render("account/signin.html", &tera::Context::new())
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(web::Html::new(html))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/account")
            .service(web::resource("/signup").name("account::signup").to(signup))
            .service(web::resource("/signin").name("account::signin").to(signin)),
    );
}
