use crate::server_error::ServerError;
use crate::template::basename_from_path;
use crate::template::{document_path, markdown::Markdown};
use actix_web::{HttpResponse, http::header::ContentType};
use std::time::UNIX_EPOCH;
use std::{fs, path::Path};

fn create_blog_index_file(blog_index_path: &str) -> Result<bool, ServerError> {
    let mut blogs: Vec<(String, String)> = Vec::new();

    blogs.push((
        "3000-01-01".to_owned(),
        format!(
            r##"---
title: Blog
description: My blog
header: blog/header.md
footer: blog/footer.md
---
"##
        ),
    ));

    let blog_dir = document_path("blog");
    for blog_dir_item in fs::read_dir(&blog_dir)? {
        let blog_files_item = blog_dir_item?;
        let basename = basename_from_path(Some(&blog_files_item.path().as_path()));
        if !basename.ends_with(".md") {
            continue;
        }

        for skip in ["footer.md", "header.md", "index.md"] {
            if basename == skip {
                continue;
            }
        }

        let mut blog = Markdown::new_from_path(&blog_files_item.path());
        if !blog.read() {
            continue;
        }
        if blog.status != "published"
            && std::env::var("APP_ENV").unwrap_or_default() != "development"
        {
            continue;
        }

        blog.content = format!(
            r##"## {}

[{}](/blog/{}-{})

{}

<a href="/blog/{}-{}" role="button" class="read-more">Read the full article</a>
"##,
            blog.title, blog.date, blog.date, blog.id, blog.ingress, blog.date, blog.id
        );

        blogs.push((blog.date, blog.content));
    }

    blogs.sort_by(|a, b| b.0.cmp(&a.0));
    let content = blogs.iter().map(|i| i.1.clone()).collect::<Vec<String>>();
    fs::write(&blog_index_path, content.join("\n"))?;

    Ok(true)
}

fn mtime(path: &str) -> u64 {
    if let Ok(m) = fs::metadata(path)
        && let Ok(m) = m.modified()
        && let Ok(m) = m.duration_since(UNIX_EPOCH)
    {
        return m.as_secs();
    }

    0
}

pub async fn get_blog_index(
    state: actix_web::web::Data<crate::AppState>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, ServerError> {
    if req.method() == actix_web::http::Method::HEAD {
        return Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .finish());
    }

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

fn get_blog_id(raw: Option<&str>) -> (String, String) {
    let raw = raw.unwrap_or_default().trim_end_matches(".html");
    if raw.len() > 11 && raw[0..10].chars().all(|c| c.is_digit(10) || c == '-') {
        return (raw[..10].to_owned(), raw[11..].to_owned());
    }

    ("".to_owned(), "".to_owned())
}

pub async fn get_blog_post(
    state: actix_web::web::Data<crate::AppState>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, ServerError> {
    if req.method() == actix_web::http::Method::HEAD {
        return Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .finish());
    }

    let mut ctx = crate::template::template_context(&req);
    let (blog_date, blog_id) = get_blog_id(req.match_info().get("blog_id"));
    let blog_path = document_path(&format!("blog/{}.md", blog_id));
    let mut article = Markdown::new_from_path(&Path::new(&blog_path));
    if !article.read() {
        return Err(ServerError::NotFound(
            "Could not find blog post.".to_owned(),
        ));
    }

    // Redirect if date does not match
    if blog_date != article.date {
        return Ok(HttpResponse::Found()
            .append_header((
                actix_web::http::header::LOCATION,
                format!("/blog/{}-{}", article.date, article.id),
            ))
            .finish());
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
