use actix_files::NamedFile;
use actix_web::{error, get, web, HttpResponse, Result};
use tera::Tera;

#[path = "utils/errors.rs"]
mod errors;
mod models;
mod views;

static STATIC_FILES: phf::Map<&'static str, &'static [u8]> = phf::phf_map! {
    "images/favicon.ico" => include_bytes!("static/images/favicon.ico"),
    "src/output.css" => include_bytes!("static/src/output.css"),
    "images/catff.png" => include_bytes!("static/images/catff.png"),
    "images/logo.png" => include_bytes!("static/images/logo.png"),
    "node_modules/preline/dist/preline.js" => include_bytes!("static/node_modules/preline/dist/preline.js"),
};

static TEMPLATE_DIR: include_dir::Dir =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/src/app/templates");
// static STATIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/app/static");

async fn serve_static(path: web::Path<String>) -> Result<NamedFile> {
    let path: std::path::PathBuf = path.into_inner().parse().unwrap();
    let file_path = path.to_str().unwrap();

    // if let Some(file) = STATIC_DIR.get_file(path.to_str().unwrap()) {
    if let Some(file) = STATIC_FILES.get(file_path) {
        // Create a temporary file and write the contents
        let temp_dir = tempfile::Builder::new().prefix("static").tempdir()?;
        let temp_path = temp_dir.path().join(path);
        if let Some(parent) = temp_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&temp_path, file)?;
        // std::fs::write(&temp_path, file.contents())?;
        Ok(NamedFile::open(temp_path)?)
    } else {
        Err(actix_web::error::ErrorNotFound("File not found")) // Implement to return page Not Found
    }
}

pub fn initialize_template() -> Tera {
    let mut tera = Tera::default();
    let mut templates: Vec<(String, String)> = Vec::new();

    fn add_templates_recursive(
        dir: &include_dir::Dir,
        base_path: &std::path::Path,
        tmpl: &mut Vec<(String, String)>,
    ) {
        for entry in dir.entries() {
            match entry {
                include_dir::DirEntry::File(file) => {
                    let full_path = base_path.join(file.path());
                    if full_path.extension().map_or(false, |ext| ext == "html") {
                        // Remove the base path to get the relative path
                        let template_path = full_path.strip_prefix(base_path).unwrap();
                        let template_name = template_path.to_str().unwrap();
                        log::debug!("Registering template: {}", template_name);
                        tmpl.push((
                            template_name.to_string(),
                            file.contents_utf8().unwrap().to_string(),
                        ));
                    }
                }
                include_dir::DirEntry::Dir(subdir) => {
                    add_templates_recursive(subdir, base_path, tmpl);
                }
            }
        }
    }

    add_templates_recursive(
        &TEMPLATE_DIR,
        std::path::Path::new("templates"),
        &mut templates,
    );
    tera.add_raw_templates(templates).unwrap();
    tera
}

#[get("/favicon.ico")]
async fn serve_favicon() -> Result<HttpResponse> {
    if let Some(favicon_data) = STATIC_FILES.get("images/favicon.ico") {
        Ok(HttpResponse::Ok()
            .content_type("image/x-icon")
            .body(favicon_data.to_vec()))
    } else {
        Err(error::ErrorNotFound("Favicon not found"))
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(views::config)
        .service(
            web::resource("/static/{filename:.*}")
                .name("static")
                .to(serve_static),
        )
        .service(serve_favicon);
    // .default_service(web::to(errors::error_handlers));
}
