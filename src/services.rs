use actix_files::Files;
use actix_web::http::StatusCode;
use actix_web::{get, web, HttpRequest, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

use crate::AppState;

const INTERNAL_SERVER_ERROR: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/internal_server_error.html"
));

const NOT_FOUND: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/not_found.html"
));

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerError {
    InternalServerError(String),
    NotFound(String),
}

impl From<tera::Error> for ServerError {
    fn from(err: tera::Error) -> ServerError {
        log::error!("type=\"tera::Error\" error=\"{:?}\"", err);
        ServerError::InternalServerError(err.to_string())
    }
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::InternalServerError(err) => {
                let html = INTERNAL_SERVER_ERROR.replace("NO_DETAILS", err);
                f.write_str(&html.as_str())
            }
            ServerError::NotFound(err) => {
                let html = NOT_FOUND.replace("NO_DETAILS", err);
                f.write_str(&html.as_str())
            }
        }
    }
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerError::InternalServerError(_) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .insert_header(("Content-Type", "text/html"))
                .body(self.to_string()),
            ServerError::NotFound(_) => HttpResponse::build(StatusCode::NOT_FOUND)
                .insert_header(("Content-Type", "text/html"))
                .body(self.to_string()),
        }
    }
}

fn template_type_from_path(path: &String) -> Result<String, ServerError> {
    for ext in &["html", "md"] {
        let full_path = format!("{}/templates/{}.{}", env!("CARGO_MANIFEST_DIR"), path, ext);
        if std::path::Path::new(&full_path).exists() {
            return Ok(ext.to_string())
        }
    }

    Err(ServerError::NotFound(format!("path=\"{}\" error=unknown_type", path)))
}

#[get("/")]
pub async fn get_index(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ServerError> {
    let ctx = crate::template::template_context(&req);
    let rendered = state.tera.render("index.html", &ctx)?;
    return Ok(HttpResponse::Ok().body(rendered));
}

#[get("/{dynamic:.*}")]
pub async fn get_markdown(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ServerError> {
    let mut ctx = crate::template::template_context(&req);
    let path = req.match_info().get("dynamic").unwrap_or("index").trim_end_matches('/').trim_start_matches("/");
    let ext = template_type_from_path(&path.to_string())?;

    if ext == "html" {
        let path = format!("{}.{}", path, ext);
        let rendered = state.tera.render(&path, &ctx)?;
        return Ok(HttpResponse::Ok().body(rendered));
    }
    if ext == "md" {
        ctx.insert("markdown_path".to_string(), &path);
        let rendered = state.tera.render("article.html", &ctx)?;
        return Ok(HttpResponse::Ok().body(rendered));
    }

    Err(ServerError::NotFound(format!("path=\"{}\" error=unknown_ext", ext)))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    let static_dir = std::env::var("THORSEN_STATIC_FILES").unwrap_or("./static".to_string());

    cfg.service(get_index);
    cfg.service(Files::new("/css", format!("{}/css", static_dir)).prefer_utf8(true));
    cfg.service(Files::new("/js", format!("{}/js", static_dir)).prefer_utf8(true));
    cfg.service(Files::new("/images", format!("{}/images", static_dir)).prefer_utf8(true));
    cfg.service(get_markdown);
}
