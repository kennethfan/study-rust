use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub done: bool,
}

