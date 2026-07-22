mod server_error;
mod services;
mod template;

#[derive(Clone)]
pub struct AppState {
    pub tera: tera::Tera,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let port: u16 = std::env::var("PORT")
        .unwrap_or("3032".to_owned())
        .parse()
        .expect("PORT= is not valid");

    tracing::info!("🚀 Listening to http://127.0.0.1:{}/ ...", port);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;
    let state = AppState {
        tera: crate::template::global_tera(),
    };

    axum::serve(listener, services::router(state)).await
}
