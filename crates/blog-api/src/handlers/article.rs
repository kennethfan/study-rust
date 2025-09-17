use axum::{Json};
use serde_json::json;

pub async fn get_articles() -> Json<serde_json::Value> {
    Json(json!([
        { "id": 1, "title": "Hello", "content": "Welcome to Rust" },
        { "id": 2, "title": "Axum", "content": "Building APIs in Rust" }
    ]))
}

