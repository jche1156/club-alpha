use axum::{routing::{get,post}, Router};
use std::net::SocketAddr;

mod showcase;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    let app = Router::new()
        .route("/", get(showcase::hello::handler))
        .route("/alpine", get(showcase::alpine::handler))
        .route("/htmx", get(showcase::htmx::read))
        .route("/htmx/edit", get(showcase::htmx::edit))
        .route("/*<name>", get(showcase::greet::handler));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
