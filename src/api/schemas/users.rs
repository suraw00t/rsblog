use std::io;

use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::{App, HttpServer, Responder, post};

use page_hunter::{Book, Page};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::models::users::User;

pub type UserPage = Page<User>;
pub type UserBook = Book<User>;

#[derive(ToSchema, MultipartForm)]
pub struct UploadPictureProfile {
    #[multipart(limit = "10mb")]
    #[schema(value_type = String, format = Binary, content_media_type = "application/octet-stream")]
    pub file: TempFile,
    #[schema(value_type = Option<String>, example = "")]
    pub rename: Option<Text<String>>,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub struct PictureProfile {
    pub rename: Option<String>,
    pub size: usize,
    pub file_name: String,
    pub content_type: String,
}
