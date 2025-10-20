use actix_web::{HttpRequest, HttpResponse, http::header::ContentType, web};
use std::collections::HashMap;
use std::path::Path;

use crate::server_error::ServerError;
use crate::template::{document_path, markdown::Markdown};

type QueryParams = HashMap<String, String>;

fn template_type_from_path(path: &String) -> Result<String, ServerError> {
    for ext in &["html", "md"] {
        let abs_path = document_path(&format!("{}/index.{}", path, ext));
        if Path::new(&abs_path).exists() {
            return Ok(ext.to_string());
        }
    }

    Err(ServerError::NotFound(format!(
        "Could not find the requested page. path=\"{}\"",
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

    if req.method() == actix_web::http::Method::HEAD {
        return Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .finish());
    }

    if ext == "html" {
        let Ok(qs) = web::Query::<QueryParams>::from_query(req.query_string()) else {
            return Err(ServerError::BadRequest("Invalid query string.".to_owned()));
        };

        ctx.insert("query".to_owned(), &qs.into_inner());

        let mut article = Markdown::default();
        article.scoped_css = format!("{}/scoped.css", article_rel_path);
        article.status = "published".to_owned();
        ctx.insert("article".to_owned(), &article);

        let article_abs_path = format!("{}/index.html", article_rel_path);
        let rendered = state.tera.render(&article_abs_path, &ctx)?;
        return Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(rendered));
    }
    if ext == "md" {
        let article_abs_path = document_path(&format!("{}/index.md", article_rel_path));
        let mut article = Markdown::new_from_path(&Path::new(&article_abs_path));
        if !article.read() {
            return Err(ServerError::NotFound(
                "Could not find article post.".to_owned(),
            ));
        }

        if article.scoped_css.len() == 0 {
            article.scoped_css = format!("{}/scoped.css", article_rel_path);
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
