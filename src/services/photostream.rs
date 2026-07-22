use super::helpers::*;
use axum::Json;
use chrono::{DateTime, Utc};
use reqwest;
use serde_with::serde_as;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Album {
    photos: Vec<Photo>,
    stream_ctag: String,
    stream_name: String,
    user_first_name: String,
    user_last_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Photo {
    batch_guid: String,
    date_created: DateTime<Utc>,
    caption: String,
    derivatives: std::collections::HashMap<String, Derivative>,
    photo_guid: String,
}

#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Derivative {
    checksum: String,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    file_size: u64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    height: u64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    width: u64,
}

async fn cached_photostream_album(icloud_id: &str) -> Result<Album, ServerError> {
    let cache_file = format!("/tmp/photostream-{}.json", icloud_id);
    let path = std::path::Path::new(&cache_file);
    if path.exists() {
        tracing::info!(
            get = "photostream_album",
            cached = true,
            icloud_id = %icloud_id,
            cache_file = %cache_file
        );
        let json = std::fs::read_to_string(path)?;
        let album: Album = serde_json::from_str(&json)?;
        return Ok(album);
    }

    tracing::info!(
        get = "photostream_album",
        cached = false,
        icloud_id = %icloud_id,
        cache_file = %cache_file
    );
    let client = reqwest::Client::new();
    let url = format!(
        "https://p118-sharedstreams.icloud.com/{}/sharedstreams/webstream",
        icloud_id
    );

    let json_str = client
        .post(url)
        .body(r#"{"streamCtag":null}"#)
        .send()
        .await?
        .text()
        .await?;

    let album: Album = serde_json::from_str(&json_str)?;
    std::fs::write(&cache_file, &json_str)?;
    Ok(album)
}

fn normalize_derivatives(album: &mut Album) {
    for i in 0..album.photos.len() {
        let mut biggest = Derivative::default();
        let mut smallest = Derivative::default();
        let mut video = Derivative::default();

        let photo = &mut album.photos[i];
        for (key, derivative) in photo.derivatives.iter() {
            if key == "720p" {
                video = derivative.clone()
            } else if key == "PosterFrame" {
                smallest = derivative.clone();
            } else {
                match key.parse::<u64>() {
                    Err(_) => {}
                    Ok(size) => {
                        if size < smallest.width || smallest.width == 0 {
                            smallest = derivative.clone();
                        }
                        if size > biggest.width {
                            biggest = derivative.clone();
                        }
                    }
                }
            }
        }

        if video.width != 0 {
            photo.derivatives.insert("video".to_owned(), video);
        }
        if smallest.width != 0 {
            photo.derivatives.insert("thumb".to_owned(), smallest);
        }
        if biggest.width != 0 {
            photo.derivatives.insert("image".to_owned(), biggest);
        }
    }
}

pub async fn get_photostream(
    State(state): State<crate::AppState>,
    Path(icloud_id): Path<String>,
    headers: HeaderMap,
    uri: Uri,
    method: Method,
) -> Result<Response, ServerError> {
    if method == Method::HEAD {
        return Ok((
            StatusCode::OK,
            [
                ("content-type", "text/html"),
                ("cache-control", "max-age=300"),
            ],
        )
            .into_response());
    }

    let mut ctx = crate::template::template_context(&headers, &uri);
    let mut album = cached_photostream_album(&icloud_id).await?;

    // Sort latest first
    album
        .photos
        .sort_by(|a, b| b.date_created.cmp(&a.date_created));
    normalize_derivatives(&mut album);

    ctx.insert("album".to_owned(), &album);

    let mut article = Markdown::default();
    article.id = icloud_id;
    article.title = format!("{}", album.stream_name);
    article.description = format!(
        "Album {} by {} {}",
        album.stream_name, album.user_first_name, album.user_last_name
    );
    article.scoped_css = "photostream/scoped.css".to_owned();
    article.status = "published".to_owned();
    ctx.insert("article".to_owned(), &article);

    let rendered = state.tera.render("photostream/index.html", &ctx)?;

    Ok((
        StatusCode::OK,
        [
            ("content-type", "text/html"),
            ("cache-control", "max-age=600"),
        ],
        rendered,
    )
        .into_response())
}

pub async fn post_webasset_urls(
    Path(icloud_id): Path<String>,
    Json(photo_guids): Json<Vec<String>>,
) -> Result<Response, ServerError> {
    for photo_guid in &photo_guids {
        if photo_guid.len() != 36 {
            return Ok((StatusCode::BAD_REQUEST, "Invalid photo GUID in list.").into_response());
        }
    }

    let url = format!(
        "https://p118-sharedstreams.icloud.com/{}/sharedstreams/webasseturls",
        icloud_id,
    );
    let client = reqwest::Client::new();
    let json_str = client
        .post(url)
        .body(serde_json::json!({"photoGuids": photo_guids}).to_string())
        .send()
        .await?
        .text()
        .await?;

    Ok((
        StatusCode::OK,
        [
            ("content-type", "application/json"),
            ("cache-control", "max-age=300"),
        ],
        json_str,
    )
        .into_response())
}
