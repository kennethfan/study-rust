mod routes;
mod models;
mod db;
mod handlers;

use axum::{Router, routing::get};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(|| async { "Hello Blog API" }))
        .merge(routes::create_routes());

    let addr = "0.0.0.0:3000".parse().unwrap();
    tracing::info!("🚀 Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

