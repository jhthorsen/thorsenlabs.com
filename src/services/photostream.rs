use actix_web::{http::header::ContentType, HttpResponse};
use chrono::{DateTime, Utc};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use Vec;

use crate::server_error::ServerError;
use crate::template::markdown::Markdown;

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
    derivatives: HashMap<String, Derivative>,
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
        log::info!(
            "get=photostream_album cached=true icloud_id=\"{}\" cache_file=\"{}\"",
            &icloud_id,
            &cache_file,
        );
        let json = std::fs::read_to_string(path)?;
        let album: Album = serde_json::from_str(&json)?;
        return Ok(album);
    }

    log::info!(
        "get=photostream_album cached=false icloud_id=\"{}\" cache_file=\"{}\"",
        &icloud_id,
        &cache_file,
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

        let photo = &mut album.photos[i];
        let keys = photo.derivatives.keys().cloned().collect::<Vec<String>>();

        for key in keys {
            if key == "720p" {
                let v = photo.derivatives.get(&key).unwrap().clone();
                photo.derivatives.insert("video".to_owned(), v);
            } else if key == "PosterFrame" {
                smallest = photo.derivatives.get(&key).unwrap().clone();
            } else {
                match key.parse::<u64>() {
                    Ok(size) => {
                        if size < smallest.width || smallest.width == 0 {
                            smallest = photo.derivatives.get(&key).unwrap().clone();
                        }
                        if size > biggest.width {
                            biggest = photo.derivatives.get(&key).unwrap().clone();
                        }
                    }
                    Err(_) => {}
                }
            }
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
    icloud_id: actix_web::web::Path<String>,
    req: actix_web::HttpRequest,
    state: actix_web::web::Data<crate::AppState>,
) -> Result<HttpResponse, ServerError> {
    if req.method() == actix_web::http::Method::HEAD {
        return Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .finish());
    }

    let mut ctx = crate::template::template_context(&req);
    let mut album = cached_photostream_album(&icloud_id).await?;

    // Sort latest first
    album
        .photos
        .sort_by(|a, b| b.date_created.cmp(&a.date_created));
    normalize_derivatives(&mut album);

    ctx.insert("album".to_owned(), &album);

    let mut article = Markdown::default();
    article.id = icloud_id.clone();
    article.title = format!("{}", album.stream_name);
    article.description = format!(
        "Album {} by {} {}",
        album.stream_name, album.user_first_name, album.user_last_name
    );
    article.scoped_css = "photostream/scoped.css".to_owned();
    article.status = "published".to_owned();
    ctx.insert("article".to_owned(), &article);

    let rendered = state.tera.render("photostream/index.html", &ctx)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .append_header(("Cache-control", "max-age=600"))
        .body(rendered))
}

pub async fn post_webasset_urls(
    icloud_id: actix_web::web::Path<String>,
    photo_guids: actix_web::web::Json<Vec<String>>,
) -> Result<HttpResponse, ServerError> {
    let photo_guids = photo_guids.into_inner();
    for photo_guid in &photo_guids {
        if photo_guid.len() != 36 {
            return Ok(HttpResponse::BadRequest().body("Invalid photo GUID in list."));
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

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .append_header(("Cache-control", "max-age=300"))
        .body(json_str))
}
