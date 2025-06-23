use std::io;

use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::{post, App, HttpServer, Responder};

use page_hunter::{Book, Page};
use utoipa::ToSchema;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::models::users::User;

pub type UserPage = Page<User>;
pub type UserBook = Book<User>;

#[derive(ToSchema, MultipartForm)]
pub struct PictureProfile {
    #[multipart(limit = "9mb")]
    #[schema(value_type = String, format = Binary, content_media_type = "application/octet-stream")]
    pub file: TempFile,
    #[schema(value_type = Option<String>, default = "")]
    pub name: Option<Text<String>>,
}
