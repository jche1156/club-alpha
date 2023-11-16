use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    let assets_path = std::env::current_dir().unwrap();
    let app = Router::new()
        .route("/", get(routes::magazine::handler))
        .route("/alpine", get(routes::alpine::handler))
        .route("/pages/*<title>", get(routes::article::handler))
        .route("/htmx", get(routes::htmx::read))
        .route("/htmx/edit", get(routes::htmx::edit))
        .route("/*<name>", get(routes::fallback::handler))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
