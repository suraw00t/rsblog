use actix_web::{error, web, Error, Responder, Result};

async fn signup(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    let html = tmpl
        .render("zaccount/signup.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(web::Html::new(html))
}

async fn signin(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
    let html = tmpl
        .render("zaccount/signin.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(web::Html::new(html))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/account")
            .service(web::resource("/signup").name("account::signup").to(signup))
            .service(web::resource("/signin").name("account::signin").to(signin)),
    );
}
