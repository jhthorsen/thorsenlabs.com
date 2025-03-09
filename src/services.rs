use actix_files::Files;
use actix_web::web;

mod arbeidsdager;
mod article;
mod events;
mod blog;
mod photostream;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let static_dir = std::env::var("THORSEN_STATIC_DIR").unwrap_or("./static".to_owned());
    cfg.service(Files::new("/css", format!("{}/css", static_dir)).prefer_utf8(true));
    cfg.service(Files::new("/fonts", format!("{}/fonts", static_dir)).prefer_utf8(true));
    cfg.service(Files::new("/js", format!("{}/js", static_dir)).prefer_utf8(true));
    cfg.service(Files::new("/images", format!("{}/images", static_dir)).prefer_utf8(true));

    match std::env::var("THORSEN_EVENT_SECRET") {
        Ok(event_secret) => {
            cfg.service(
                web::resource(format!("/events/{}/push", event_secret))
                    .route(web::post().to(events::git_push)),
            );
        }
        Err(_) => { },
    }

    cfg.service(
        web::resource("/arbeidsdager/table/{year}")
            .route(web::get().to(arbeidsdager::get_arbeidsdager_table))
            .route(web::head().to(arbeidsdager::get_arbeidsdager_table)),
    );

    cfg.service(
        web::resource("/blog")
            .route(web::get().to(blog::get_blog_index))
            .route(web::head().to(blog::get_blog_index)),
    );

    cfg.service(
        web::resource("/blog.html")
            .route(web::get().to(blog::get_blog_index))
            .route(web::head().to(blog::get_blog_index)),
    );

    cfg.service(
        web::resource("/blog/{blog_id}")
            .route(web::get().to(blog::get_blog_post))
            .route(web::head().to(blog::get_blog_post)),
    );

    cfg.service(
        web::resource("/blog/{blog_id}.html")
            .route(web::get().to(blog::get_blog_post))
            .route(web::head().to(blog::get_blog_post)),
    );

    cfg.service(
        web::resource("/photostream/{icloud_id}")
            .route(web::get().to(photostream::get_photostream))
            .route(web::head().to(photostream::get_photostream)),
    );

    cfg.service(
        web::resource("/photostream/{icloud_id}/webassets")
            .route(web::post().to(photostream::post_webasset_urls)),
    );

    cfg.service(
        web::resource("/{article:.*}")
            .route(web::get().to(article::get_article))
            .route(web::head().to(article::get_article)),
    );
}
