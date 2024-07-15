use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use std::collections::HashMap;
use std::path::Path;

use crate::server_error::ServerError;
use crate::template::{document_path, markdown::Markdown};

type QueryParams = HashMap<String, String>;

fn template_type_from_path(path: &String) -> Result<String, ServerError> {
    for ext in &["html", "md"] {
        let full_path = document_path(&format!("{}.{}", path, ext));
        if Path::new(&full_path).exists() {
            return Ok(ext.to_string());
        }
    }

    Err(ServerError::NotFound(format!(
        "path=\"{}\" error=\"Type not found\"",
        path
    )))
}

pub async fn get_article(
    state: web::Data<crate::AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ServerError> {
    let article_rel_path = req
        .match_info()
        .get("article")
        .unwrap_or("index")
        .trim_start_matches("/")
        .trim_end_matches("/")
        .trim_end_matches(".html")
        .trim_end_matches(".md");

    let mut ctx = crate::template::template_context(&req);
    let ext = template_type_from_path(&article_rel_path.to_owned())?;
    if ext == "html" {
        let qs = web::Query::<QueryParams>::from_query(req.query_string()).unwrap();
        ctx.insert("query".to_owned(), &qs.into_inner());

        let article_abs_path = format!("{}.{}", article_rel_path, ext);
        let rendered = state.tera.render(&article_abs_path, &ctx)?;
        return Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(rendered));
    }
    if ext == "md" {
        let article_abs_path = document_path(&format!("{}.md", article_rel_path));
        let mut article = Markdown::new_from_path(&Path::new(&article_abs_path));
        if !article.read() {
            return Err(ServerError::NotFound(
                "Could not find article post.".to_owned(),
            ));
        }

        ctx.insert("article".to_owned(), &article);

        let rendered = state.tera.render("layouts/article.html", &ctx)?;
        return Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(rendered));
    }

    Err(ServerError::NotFound(format!(
        "path=\"{}\" error=unknown_ext",
        ext
    )))
}
