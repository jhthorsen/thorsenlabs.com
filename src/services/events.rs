use super::helpers::*;
use axum::body::Bytes;

pub async fn git_push(body: Bytes) -> Result<Response, ServerError> {
    let update_file = format!("/tmp/git-push-thorsenlabs.com.json");
    std::fs::write(&update_file, body.to_vec())?;

    Ok((
        StatusCode::OK,
        [("content-type", "application/json")],
        "{\"updated\":true}",
    )
        .into_response())
}
