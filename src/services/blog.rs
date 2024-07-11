use std::time::UNIX_EPOCH;
use std::{fs, path::Path};

use crate::server_error::ServerError;
use crate::template::{markdown::Markdown, document_path};

fn create_blog_list_file() -> Result<bool, ServerError> {
    let mut blogs: Vec<(String, String)> = Vec::new();

    let blog_dir = document_path("blog");
    for blog_dir_item in fs::read_dir(&blog_dir)? {
        let blog_files_item = blog_dir_item?;
        let basename = blog_files_item.file_name().into_string().unwrap();
        if !basename.starts_with("2") || !basename.ends_with(".md") {
            continue;
        }

        let mut blog = Markdown::new_from_path(&blog_files_item.path());
        if blog.read() {
            blog.content = format!(
                "## {}\n\n<a href=\"/blog/{}\">{}</a>\n\n{}\n",
                blog.title, blog.id, blog.date, blog.ingress,
            );

            blogs.push((blog.date, blog.content));
        }
    }

    blogs.sort_by(|a, b| b.0.cmp(&a.0));
    let content = blogs.iter().map(|i| i.1.clone()).collect::<Vec<String>>();
    fs::write(format!("{}/list.md", &blog_dir), content.join("\n"))?;

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
) -> Result<actix_web::HttpResponse, ServerError> {
    let mut ctx = crate::template::template_context(&req);
    ctx.insert("updated".to_owned(), &false);

    let blog_dir = document_path("blog");
    if mtime(blog_dir.as_str()) > mtime(format!("{}/list.md", &blog_dir).as_str()) {
        ctx.insert("updated".to_owned(), &create_blog_list_file());
    }

    let rendered = state.tera.render("blog/index.html", &ctx)?;
    return Ok(actix_web::HttpResponse::Ok().body(rendered));
}

pub async fn get_blog_post(
    state: actix_web::web::Data<crate::AppState>,
    req: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, ServerError> {
    let mut ctx = crate::template::template_context(&req);

    let blog_id = req.match_info().get("blog_id").unwrap_or("not_found");
    let blog_path = document_path(&format!("blog/{}.md", blog_id));
    let mut blog = Markdown::new_from_path(&Path::new(&blog_path));
    if !blog.read() {
        return Err(ServerError::NotFound(
            "Could not find blog post.".to_owned(),
        ));
    }

    ctx.insert("blog".to_owned(), &blog);

    let rendered = state.tera.render("blog/entry.html", &ctx)?;
    return Ok(actix_web::HttpResponse::Ok().body(rendered));
}
