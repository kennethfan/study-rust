#!/bin/bash

cargo update

cargo build --workspace

cargo run -p todo-cli -- add "Learn Rust ownership"
cargo run -p todo-cli -- add "Finish todo-cli project"
cargo run -p todo-cli -- list
