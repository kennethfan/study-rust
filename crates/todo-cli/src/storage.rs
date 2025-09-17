use std::fs;
use std::path::Path;
use crate::model::Todo;

const FILE_PATH: &str = "todos.json";

pub fn load_todos() -> Vec<Todo> {
    if Path::new(FILE_PATH).exists() {
        let data = fs::read_to_string(FILE_PATH).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        vec![]
    }
}

pub fn save_todos(todos: &Vec<Todo>) {
    let data = serde_json::to_string_pretty(todos).unwrap();
    fs::write(FILE_PATH, data).unwrap();
}
