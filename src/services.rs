use actix_files::Files;
use actix_web::web;

use index::get_index;
use wildcard::get_wildcard;

mod index;
mod wildcard;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let static_dir = std::env::var("THORSEN_STATIC_FILES").unwrap_or("./static".to_string());

    cfg.service(web::resource("/").route(web::get().to(get_index)));
    cfg.service(Files::new("/css", format!("{}/css", static_dir)).prefer_utf8(true));
    cfg.service(Files::new("/js", format!("{}/js", static_dir)).prefer_utf8(true));
    cfg.service(Files::new("/images", format!("{}/images", static_dir)).prefer_utf8(true));
    cfg.service(web::resource("/{wildcard:.*}").route(web::get().to(get_wildcard)));
}
