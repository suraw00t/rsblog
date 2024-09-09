use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

#[get("/")]
async fn root() -> HttpResponse {
    let response = json!({
        "message": "Received your data",
        "data": 0,
        "additional": "Some extra info"
    });

    HttpResponse::Ok().json(response)
}

/// This handler uses json extractor
#[post("/extractor")]
async fn index(item: web::Json<MyObj>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

/// This handler uses json extractor with limit
async fn extract_item(item: web::Json<MyObj>, req: HttpRequest) -> HttpResponse {
    println!("request: {req:?}");
    println!("model: {item:?}");

    HttpResponse::Ok().json(item.0) // <- send json response
}

/// This handler manually load request payload and parse json object
#[post("/manual")]
async fn index_manual(body: web::Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<MyObj>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(root)
        .service(index)
        .service(
            web::resource("/extractor2")
                .app_data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
                .route(web::post().to(extract_item)),
        )
        .service(index_manual);
}

// #[cfg(test)]
// mod tests {
//     use actix_web::{body::to_bytes, dev::Service, http, test};

//     use super::*;

//     #[actix_web::test]
//     async fn test_index() {
//         let app =
//             test::init_service(App::new().service(web::resource("/").route(web::post().to(index))))
//                 .await;

//         let req = test::TestRequest::post()
//             .uri("/")
//             .set_json(MyObj {
//                 name: "my-name".to_owned(),
//                 number: 43,
//             })
//             .to_request();
//         let resp = app.call(req).await.unwrap();

//         assert_eq!(resp.status(), http::StatusCode::OK);

//         let body_bytes = to_bytes(resp.into_body()).await.unwrap();
//         assert_eq!(body_bytes, r#"{"name":"my-name","number":43}"#);
//     }
// }
