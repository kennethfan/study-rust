use minigrep::Config;
use std::{env, process};

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    dbg!(&config);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}



