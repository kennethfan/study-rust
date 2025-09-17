use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub content: String,
}

