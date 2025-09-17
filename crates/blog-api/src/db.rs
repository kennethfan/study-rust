use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::env;

pub async fn get_db_pool() -> SqlitePool {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePoolOptions::new()
        .connect(&url)
        .await
        .expect("Failed to connect to DB")
}

