use actix_web::{middleware::Logger, web, App, HttpServer};

mod server_error;
mod services;
mod template;

pub struct AppState {
    pub tera: tera::Tera,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port: u16 = std::env::var("PORT")
        .unwrap_or("3032".to_string())
        .parse()
        .unwrap();

    log::info!("🚀 Listening to http://127.0.0.1:{}/ ...", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                tera: crate::template::build_tera().clone(),
            }))
            .configure(services::configure)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
