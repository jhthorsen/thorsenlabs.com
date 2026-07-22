use crate::template::global_tera;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerError {
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
}

impl From<reqwest::Error> for ServerError {
    fn from(err: reqwest::Error) -> ServerError {
        tracing::error!(error_type = "reqwest", error = ?err);
        ServerError::InternalServerError(err.to_string())
    }
}

impl From<serde_json::Error> for ServerError {
    fn from(err: serde_json::Error) -> ServerError {
        tracing::error!(error_type = "serde_json", error = ?err);
        ServerError::InternalServerError(err.to_string())
    }
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> ServerError {
        tracing::error!(error_type = "std::io", error = ?err);
        ServerError::InternalServerError(err.to_string())
    }
}

impl From<tera::Error> for ServerError {
    fn from(err: tera::Error) -> ServerError {
        tracing::error!(error_type = "tera", error = ?err);
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

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status = match self {
            ServerError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServerError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::NotFound(_) => StatusCode::NOT_FOUND,
        };

        (status, [("content-type", "text/html")], self.to_string()).into_response()
    }
}
