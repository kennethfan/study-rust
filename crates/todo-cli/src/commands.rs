use std::env;
use crate::model::Todo;
use crate::storage::{load_todos, save_todos};

pub fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let cmd = args.next().ok_or("No command")?;

    match cmd.as_str() {
        "add" => {
            let text = args.collect::<Vec<_>>().join(" ");
            if text.is_empty() {
                return Err("Missing todo text".into());
            }
            let mut todos = load_todos();
            let id = if todos.is_empty() { 1 } else { todos.last().unwrap().id + 1 };
            todos.push(Todo { id, text, done: false });
            save_todos(&todos);
            println!("✅ Added todo #{id}");
        }
        "list" => {
            let todos = load_todos();
            for t in todos {
                let status = if t.done { "[x]" } else { "[ ]" };
                println!("{} {} {}", t.id, status, t.text);
            }
        }
        "remove" => {
            let id = args.next().ok_or("Missing todo id")?;
            let id = id.parse::<u32>().map_err(|_| "Invalid todo id")?;
            let mut todos = load_todos();
            todos.retain(|t| t.id != id);
            save_todos(&todos);
        }
        "done" => {
            let id = args.next().ok_or("Missing todo id")?;
            let id = id.parse::<u32>().map_err(|_| "Invalid todo id")?;
            let mut todos = load_todos();
            match todos.iter_mut().find(|t| t.id == id) {
                Some(todo) => todo.done = true,
                None => println!("Todo with id {} not found", id),
            }
            save_todos(&todos);
        }
        _ => return Err("Unknown command".into())
    }
    Ok(())
}
