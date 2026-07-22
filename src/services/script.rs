use super::helpers::*;

pub async fn get_script(Path(name): Path<String>) -> Result<Response, ServerError> {
    let Some(path) = name.strip_suffix(".js") else {
        return Err(ServerError::NotFound(format!("{name}")));
    };

    let (age, path) = if let Some(r) = path.rsplit_once("@") {
        (86400, r.0)
    } else if let Some(r) = path.rsplit_once("+") {
        (31536000, r.0)
    } else {
        (86400, name.as_str())
    };

    let path = match path.contains('/') {
        true => format!("{path}.js"),
        false => format!("{path}/{path}.js"),
    };

    Ok((
        StatusCode::OK,
        [
            (
                "content-type",
                "application/javascript; charset=utf-8".to_string(),
            ),
            ("cache-control", format!("public, max-age={age}")),
        ],
        std::fs::read_to_string(&document_path(&path))
            .map_err(|_| ServerError::NotFound(format!("Could not read script {}", name)))?,
    )
        .into_response())
}
