#!/bin/bash

set -x
export RUST_BACKTRACE=1

base_path=$(dirname $(dirname $0))

#cargo update
#
#cargo build --workspace
#
#export TODO_FILE_PATH=${base_path}/data/todos.json
#cargo run -p todo-cli -- add "Learn Rust ownership"
#cargo run -p todo-cli -- add "Finish todo-cli project"
#cargo run -p todo-cli -- list


#cargo run -p minigrep -- "rust" ${base_path}/data/sample111.txt
#cargo run -p minigrep -- "们" ${base_path}/data/sample.txt

export WEB_ROOT=${base_path}/data/
cargo run -p single-thread-web-server
