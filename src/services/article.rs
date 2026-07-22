use super::helpers::*;
use std::path::Path as FilePath;

type QueryParams = std::collections::HashMap<String, String>;

fn template_type_from_path(path: &String) -> Result<String, ServerError> {
    for ext in &["html", "md"] {
        let abs_path = document_path(&format!("{}/index.{}", path, ext));
        if FilePath::new(&abs_path).exists() {
            return Ok(ext.to_string());
        }
    }

    Err(ServerError::NotFound(format!(
        "Could not find the requested page. path=\"{}\"",
        path
    )))
}

pub async fn get_article(
    State(state): State<crate::AppState>,
    article: Option<Path<String>>,
    headers: HeaderMap,
    uri: Uri,
    method: Method,
) -> Result<Response, ServerError> {
    let article = article.map(|Path(article)| article).unwrap_or_default();
    let article_rel_path = article
        .as_str()
        .trim_start_matches("/")
        .trim_end_matches("/")
        .trim_end_matches(".html")
        .trim_end_matches(".md");

    let mut ctx = crate::template::template_context(&headers, &uri);
    let ext = template_type_from_path(&article_rel_path.to_owned())?;

    if method == Method::HEAD {
        return Ok(StatusCode::OK.into_response());
    }

    if ext == "html" {
        let Ok(qs) = Query::<QueryParams>::try_from_uri(&uri) else {
            return Err(ServerError::BadRequest("Invalid query string".to_owned()));
        };

        ctx.insert("query".to_owned(), &qs.0);

        let mut article = Markdown::default();
        article.scoped_css = format!("{}/scoped.css", article_rel_path);
        article.status = "published".to_owned();
        ctx.insert("article".to_owned(), &article);

        let article_abs_path = format!("{}/index.html", article_rel_path);
        let rendered = state.tera.render(&article_abs_path, &ctx)?;
        return Ok(Html(rendered).into_response());
    }
    if ext == "md" {
        let article_abs_path = document_path(&format!("{}/index.md", article_rel_path));
        let mut article = Markdown::new_from_path(&FilePath::new(&article_abs_path));
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
        return Ok(Html(rendered).into_response());
    }

    Err(ServerError::NotFound(format!(
        "path=\"{}\" error=unknown_ext",
        ext
    )))
}
