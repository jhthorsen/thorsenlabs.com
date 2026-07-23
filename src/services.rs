mod arbeidsdager;
mod article;
mod blog;
mod events;
mod files;
mod helpers;
mod network;
mod photostream;

use axum::Router;
use axum::routing::{get, post};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub fn router(state: crate::AppState) -> Router {
    let static_dir = std::env::var("THORSEN_STATIC_DIR").unwrap_or("./static".to_owned());
    let router = Router::new()
        .nest_service("/fonts", ServeDir::new(format!("{}/fonts", static_dir)))
        .nest_service("/images", ServeDir::new(format!("{}/images", static_dir)))
        .route("/css/{*name}", get(files::get_style))
        .route("/js/{*name}", get(files::get_script))
        .route(
            "/arbeidsdager/table/{year}",
            get(arbeidsdager::get_arbeidsdager_table),
        )
        .route("/blog", get(blog::get_blog_index))
        .route("/blog.html", get(blog::get_blog_index))
        .route("/blog/{blog_id}", get(blog::get_blog_post))
        .route(
            "/photostream/{icloud_id}",
            get(photostream::get_photostream),
        )
        .route(
            "/photostream/{icloud_id}/webassets",
            post(photostream::post_webasset_urls),
        )
        .route("/v1/gethostbyname", post(network::post_gethostbyname))
        .route("/v1/is-in-net", post(network::post_is_in_net))
        .route("/", get(article::get_article))
        .route("/{*article}", get(article::get_article))
        .layer(TraceLayer::new_for_http());

    let router = match std::env::var("THORSEN_EVENT_SECRET") {
        Ok(event_secret) => router.route(
            &format!("/events/{event_secret}/push"),
            post(events::git_push),
        ),
        Err(_) => router,
    };

    router.with_state(state)
}
