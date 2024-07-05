use actix_web::{get, web, HttpRequest, HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use serde::{Serialize, Deserialize};

use crate::AppState;
use crate::template::relative_to_markdown_file;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerError {
    UserError(String),
}

impl From<tera::Error> for ServerError {
    fn from(err: tera::Error) -> ServerError {
        log::error!("type=\"tera::Error\" error=\"{:?}\"", err);
        ServerError::UserError(err.to_string())
    }
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::UserError(err) => f.write_str(err),
        }
    }
}

impl ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServerError::UserError(_err) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/")]
pub async fn get_index(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ServerError> {
    let ctx = crate::template::template_context(req);
    let rendered = state.tera.render("index.html", &ctx)?;
    return Ok(HttpResponse::Ok().body(rendered));
}

#[get("/{markdown:.*}")]
pub async fn get_markdown(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ServerError> {
    let path = req.match_info().get("markdown");
    let path = relative_to_markdown_file(path.unwrap_or("index"));
    let mut ctx = crate::template::template_context(req);

    match path {
        Ok(path) => {
            ctx.insert("markdown_path".to_string(), &path);
            let rendered = state.tera.render("article.html", &ctx)?;
            return Ok(HttpResponse::Ok().body(rendered));
        }
        Err(err) => {
            ctx.insert("error".to_string(), &err.to_string());
            let rendered = state.tera.render("not_found.html", &ctx)?;
            return Ok(HttpResponse::NotFound().body(rendered));
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_index);
    cfg.service(get_markdown);
}
