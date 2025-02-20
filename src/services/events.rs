use actix_web::{web, http::header::ContentType, HttpResponse};

use crate::server_error::ServerError;

pub async fn git_push(
    body: web::Bytes,
) -> Result<HttpResponse, ServerError> {
    let update_file = format!("/tmp/pushed");
    std::fs::write(&update_file, body.to_vec())?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("{\"updated\":true}"))
}
