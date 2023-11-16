use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    let app = Router::new()
        .route("/", get(routes::hello::handler))
        .route("/alpine", get(routes::alpine::handler))
        .route("/pages", get(routes::magazine::handler))
        .route("/htmx", get(routes::htmx::read))
        .route("/htmx/edit", get(routes::htmx::edit))
        .route("/*<name>", get(routes::greet::handler));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
