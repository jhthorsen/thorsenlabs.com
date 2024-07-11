use actix_web::{web, HttpRequest, HttpResponse};
use std::path::Path;

use crate::server_error::ServerError;
use crate::template::document_path;

fn template_type_from_path(path: &String) -> Result<String, ServerError> {
    for ext in &["html", "md"] {
        let full_path = document_path(&format!("{}.{}", path, ext));
        if Path::new(&full_path).exists() {
            return Ok(ext.to_string());
        }
    }

    Err(ServerError::NotFound(format!(
        "path=\"{}\" error=\"Type not found\"",
        path
    )))
}

pub async fn get_wildcard(
    state: web::Data<crate::AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ServerError> {
    let path = req
        .match_info()
        .get("wildcard")
        .unwrap_or("index")
        .trim_end_matches('/')
        .trim_start_matches("/");

    let mut ctx = crate::template::template_context(&req);
    let ext = template_type_from_path(&path.to_owned())?;
    if ext == "html" {
        let path = format!("{}.{}", path, ext);
        let rendered = state.tera.render(&path, &ctx)?;
        return Ok(HttpResponse::Ok().body(rendered));
    }
    if ext == "md" {
        ctx.insert("markdown_path".to_owned(), &path);
        let rendered = state.tera.render("article.html", &ctx)?;
        return Ok(HttpResponse::Ok().body(rendered));
    }

    Err(ServerError::NotFound(format!(
        "path=\"{}\" error=unknown_ext",
        ext
    )))
}
