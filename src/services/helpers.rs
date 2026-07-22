pub use crate::server_error::ServerError;
pub use crate::template::{basename_from_path, document_path, markdown::Markdown};
pub use axum::extract::{Path, Query, State};
pub use axum::http::{HeaderMap, Method, StatusCode, Uri};
pub use axum::response::{IntoResponse, Response};
pub use serde::{Deserialize, Serialize};

pub fn cache_control_header(headers: &HeaderMap, max_age: u32) -> (&'static str, String) {
    let x_nonce = crate::template::header_value_to_string(headers.get("x-nonce"));
    if x_nonce.is_empty() && max_age > 0 {
        ("cache-control", format!("public, max-age={max_age}"))
    } else {
        (
            "cache-control",
            "private, no-cache, no-store, must-revalidate".to_string(),
        )
    }
}

#[inline]
pub fn ct(ct: &str) -> (&'static str, String) {
    ("content-type", ct.to_string())
}
