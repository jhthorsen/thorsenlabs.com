use actix_web::{http::header::ContentType, HttpResponse};
use std::time::UNIX_EPOCH;
use std::{fs, path::Path};

use crate::server_error::ServerError;
use crate::template::{document_path, markdown::Markdown};

fn create_blog_index_file(blog_index_path: &str) -> Result<bool, ServerError> {
    let mut blogs: Vec<(String, String)> = Vec::new();

    blogs.push(("3000-01-01".to_owned(), format!(
                r##"---
header: blog/header.md
footer: blog/footer.md
---
"##)));

    let blog_dir = document_path("blog");
    for blog_dir_item in fs::read_dir(&blog_dir)? {
        let blog_files_item = blog_dir_item?;
        let basename = blog_files_item.file_name().into_string().unwrap();
        if !basename.starts_with("2") || !basename.ends_with(".md") {
            continue;
        }

        let mut blog = Markdown::new_from_path(&blog_files_item.path());
        if !blog.read() {
            continue;
        }
        if blog.status != "published" {
            continue;
        }

        blog.content = format!(
            r##"## {}

[{}](/blog/{})

{}
"##,
            blog.title, blog.date, blog.id, blog.ingress
        );

        blogs.push((blog.date, blog.content));
    }

    blogs.sort_by(|a, b| b.0.cmp(&a.0));
    let content = blogs.iter().map(|i| i.1.clone()).collect::<Vec<String>>();
    fs::write(&blog_index_path, content.join("\n"))?;

    Ok(true)
}

fn mtime(path: &str) -> u64 {
    let metadata = fs::metadata(path);
    match metadata {
        Ok(metadata) => metadata
            .modified()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        Err(_) => 0,
    }
}

pub async fn get_blog_index(
    state: actix_web::web::Data<crate::AppState>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, ServerError> {
    let mut ctx = crate::template::template_context(&req);

    let blog_dir = document_path("blog");
    let blog_index_path = &format!("{}/index.md", &blog_dir);
    if mtime(blog_dir.as_str()) > mtime(format!("{}/list.md", &blog_dir).as_str()) {
        ctx.insert(
            "updated".to_owned(),
            &create_blog_index_file(&blog_index_path)?,
        );
    }

    let mut article = Markdown::new_from_path(&Path::new(&blog_index_path));
    if !article.read() {
        return Err(ServerError::NotFound(
            "Blog index was not generated.".to_owned(),
        ));
    }

    article.scoped_css = "blog/scoped.css".to_owned();
    ctx.insert("article".to_owned(), &article);
    let rendered = state.tera.render("layouts/article.html", &ctx)?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}

pub async fn get_blog_post(
    state: actix_web::web::Data<crate::AppState>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, ServerError> {
    let mut ctx = crate::template::template_context(&req);

    let blog_id = req
        .match_info()
        .get("blog_id")
        .unwrap_or("not_found")
        .trim_end_matches(".html");
    let blog_path = document_path(&format!("blog/{}.md", blog_id));
    let mut article = Markdown::new_from_path(&Path::new(&blog_path));
    if !article.read() {
        return Err(ServerError::NotFound(
            "Could not find blog post.".to_owned(),
        ));
    }

    if article.scoped_css.len() == 0 {
        article.scoped_css = "blog/scoped.css".to_owned();
    }

    ctx.insert("article".to_owned(), &article);

    let rendered = state.tera.render("blog/entry.html", &ctx)?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}
