use actix_web::{web, App, HttpServer};

mod services;
mod template;

pub struct AppState {
    pub tera: tera::Tera,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let port: u16 = std::env::var("PORT")
        .unwrap_or("3032".to_string())
        .parse()
        .unwrap();

    println!("🚀 Listening to http://127.0.0.1:{}/ ...", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                tera: crate::template::build_tera().clone(),
            }))
            .configure(services::configure)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
