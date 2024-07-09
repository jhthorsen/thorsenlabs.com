use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

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

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> ServerError {
        log::error!("type=\"std::io::Error\" error=\"{:?}\"", err);
        ServerError::InternalServerError(err.to_string())
    }
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
