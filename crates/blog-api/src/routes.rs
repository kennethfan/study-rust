use axum::{Router, routing::get};
use crate::handlers::{user::get_users, article::get_articles};

pub fn create_routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/articles", get(get_articles))
}

