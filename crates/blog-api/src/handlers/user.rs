use axum::{Json};
use serde_json::json;

pub async fn get_users() -> Json<serde_json::Value> {
    Json(json!([
        { "id": 1, "username": "alice" },
        { "id": 2, "username": "bob" }
    ]))
}

