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

    let addr: std::net::SocketAddr = "0.0.0.0:3000".parse().unwrap();
    tracing::info!("🚀 Server running at http://{}", addr);

    // 使用 tokio 的 TcpListener
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

