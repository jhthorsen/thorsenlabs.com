use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

use crate::template::global_tera;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerError {
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
}

impl From<reqwest::Error> for ServerError {
    fn from(err: reqwest::Error) -> ServerError {
        log::error!("type=\"reqwest\" error=\"{:?}\"", err);
        ServerError::InternalServerError(err.to_string())
    }
}

impl From<serde_json::Error> for ServerError {
    fn from(err: serde_json::Error) -> ServerError {
        log::error!("type=\"serde_json\" error=\"{:?}\"", err);
        ServerError::InternalServerError(err.to_string())
    }
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> ServerError {
        log::error!("type=\"std::io\" error=\"{:?}\"", err);
        ServerError::InternalServerError(err.to_string())
    }
}

impl From<tera::Error> for ServerError {
    fn from(err: tera::Error) -> ServerError {
        log::error!("type=\"tera\" error=\"{:?}\"", err);
        ServerError::InternalServerError(err.to_string())
    }
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::BadRequest(err) => {
                let mut ctx = tera::Context::new();
                ctx.insert("error", &err);
                let html = global_tera()
                    .render("bad_request.html", &ctx)
                    .expect("Global tera");
                f.write_str(&html.as_str())
            }
            ServerError::InternalServerError(err) => {
                let mut ctx = tera::Context::new();
                ctx.insert("error", &err);
                let html = global_tera()
                    .render("internal_server_error.html", &ctx)
                    .expect("Global tera");
                f.write_str(&html.as_str())
            }
            ServerError::NotFound(err) => {
                let mut ctx = tera::Context::new();
                ctx.insert("error", &err);
                let html = global_tera()
                    .render("not_found.html", &ctx)
                    .expect("Global tera");
                f.write_str(&html.as_str())
            }
        }
    }
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerError::BadRequest(_) => HttpResponse::build(StatusCode::BAD_REQUEST)
                .insert_header(("Content-Type", "text/html"))
                .body(self.to_string()),
            ServerError::InternalServerError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .insert_header(("Content-Type", "text/html"))
                    .body(self.to_string())
            }
            ServerError::NotFound(_) => HttpResponse::build(StatusCode::NOT_FOUND)
                .insert_header(("Content-Type", "text/html"))
                .body(self.to_string()),
        }
    }
}
