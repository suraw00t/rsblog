use actix_files::NamedFile;
use actix_web::{web, Result};
use include_dir::{include_dir, Dir};
use std::path::{Path, PathBuf};
use tempfile::Builder;
use tera::Tera;

#[path = "utils/errors.rs"]
mod errors;
mod views;

static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/app/templates");
static STATIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/app/static");

async fn serve_static(path: web::Path<String>) -> Result<NamedFile> {
    let path: PathBuf = path.into_inner().parse().unwrap();
    if let Some(file) = STATIC_DIR.get_file(path.to_str().unwrap()) {
        // Create a temporary file and write the contents
        let temp_dir = Builder::new().prefix("static").tempdir()?;
        let temp_path = temp_dir.path().join(path);
        if let Some(parent) = temp_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&temp_path, file.contents())?;
        Ok(NamedFile::open(temp_path)?)
    } else {
        Err(actix_web::error::ErrorNotFound("File not found"))
    }
}
pub fn initialize_template() -> Tera {
    let mut tera = Tera::default();

    fn add_templates_recursive(dir: &Dir, base_path: &Path, tera: &mut Tera) {
        for entry in dir.entries() {
            match entry {
                include_dir::DirEntry::File(file) => {
                    let full_path = base_path.join(file.path());
                    if full_path.extension().map_or(false, |ext| ext == "html") {
                        // Remove the base path to get the relative path
                        let template_path = full_path.strip_prefix(base_path).unwrap();
                        let template_name = template_path.to_str().unwrap();
                        log::debug!("Registering template: {}", template_name);
                        tera.add_raw_template(template_name, file.contents_utf8().unwrap())
                            .unwrap();
                    }
                }
                include_dir::DirEntry::Dir(subdir) => {
                    add_templates_recursive(subdir, base_path, tera);
                }
            }
        }
    }

    add_templates_recursive(&TEMPLATE_DIR, Path::new("templates"), &mut tera);
    tera
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(views::config)
        .service(web::resource("/static/{filename:.*}").to(serve_static));
    // .default_service(web::to(errors::error_handlers));
}
