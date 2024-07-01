use actix_web::{get, web, HttpRequest, HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use serde::{Serialize, Deserialize};

use crate::AppState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerError {
    UserError(String),
}

impl From<tera::Error> for ServerError {
    fn from(err: tera::Error) -> ServerError {
        println!("type=\"Residential\" {:?}", err);
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

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_index);
}
