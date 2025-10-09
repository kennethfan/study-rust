use std::fs;
use std::path::Path;
use crate::model::Todo;

const DEFAULT_FILE_PATH: &str = "todos.json";
const FILE_PATH_ENV: &str = "TODO_FILE_PATH";
fn get_file_path() -> String {
    std::env::var(FILE_PATH_ENV.to_string()).unwrap_or(DEFAULT_FILE_PATH.to_string())
}

pub fn load_todos() -> Vec<Todo> {
    let file_path = get_file_path();
    if Path::new(&file_path).exists() {
        let data = fs::read_to_string(file_path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        vec![]
    }
}

pub fn save_todos(todos: &Vec<Todo>) {
    let data = serde_json::to_string_pretty(todos).unwrap();
    let file_path = get_file_path();
    fs::write(file_path, data).unwrap();
}
