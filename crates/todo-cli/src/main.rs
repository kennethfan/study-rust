mod model;
mod storage;
mod commands;

use commands::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
