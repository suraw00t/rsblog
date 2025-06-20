use actix_web::{error, web, Error, Responder, Result};
use mongodb::bson::doc;
use std::collections::HashMap;

use crate::app::models::page_view::PageView;
use crate::common::db::get_db;

// #[get("/")]
async fn index(
    tmpl: web::Data<tera::Tera>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, Error> {
    let html = if let Some(name) = query.get("name") {
        // submitted form
        let mut ctx = tera::Context::new();
        ctx.insert("name", name);
        ctx.insert("text", "Welcome!");
        tmpl.render("user.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    } else {
        let collection = get_db().collection::<PageView>("page_views");
        // Update or insert page view count
        let filter = doc! { "path": "/" };
        let update = doc! {
            "$inc": { "views": 1 },
            "$setOnInsert": { "path": "/" }
        };

        match collection
            .find_one_and_update(filter, update)
            .return_document(mongodb::options::ReturnDocument::After)
            .await
        {
            Ok(Some(page_view)) => {
                let ctx = tera::Context::from_serialize(page_view)
                    .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;
                tmpl.render("index.html", &ctx)
                    .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
            }
            Ok(None) => {
                let page_view = PageView {
                    path: "/".to_string(),
                    views: 0,
                };
                match collection.insert_one(&page_view).await {
                    Ok(_) => {
                        let ctx = tera::Context::from_serialize(&page_view)
                            .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;
                        tmpl.render("index.html", &ctx)
                            .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
                    }
                    Err(e) => return Err(error::ErrorInternalServerError(e.to_string())),
                }
            }
            Err(e) => return Err(error::ErrorInternalServerError(e.to_string())),
        }
    };

    Ok(web::Html::new(html))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").name("root::index").to(index));
}
