use super::helpers::*;

async fn get_file(ct: &'static str, name: &str) -> Result<Response, ServerError> {
    let (path, ext) = name.rsplit_once(".").unwrap_or((name, "txt"));
    let Some((path, _)) = path.rsplit_once("@") else {
        return Err(ServerError::NotFound(format!("File not found: {name}")));
    };

    const MAX_AGE: u32 = 86400 * 7;
    let path = match path.contains('/') {
        true => format!("{path}.{ext}"),
        false => format!("{path}/{path}.{ext}"),
    };

    Ok((
        StatusCode::OK,
        [
            ("content-type", ct.to_string()),
            ("cache-control", format!("public, max-age={MAX_AGE}")),
        ],
        std::fs::read_to_string(&document_path(&path))
            .map_err(|_| ServerError::NotFound(format!("Could not read script {}", name)))?,
    )
        .into_response())
}

pub async fn get_script(Path(name): Path<String>) -> Result<Response, ServerError> {
    get_file("application/javascript; charset=utf-8", &name).await
}

pub async fn get_style(Path(name): Path<String>) -> Result<Response, ServerError> {
    get_file("text/css", &name).await
}
