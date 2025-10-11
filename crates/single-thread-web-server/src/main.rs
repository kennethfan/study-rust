use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use async_std::task;

static mut WEB_ROOT: String = String::new();

#[async_std::main]
async fn main() {
    println!("Hello, world!");

    unsafe {
        WEB_ROOT = env::var("WEB_ROOT".to_string()).unwrap_or("".to_string());
    }

    let listener = TcpListener::bind("127.0.0.1:6000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream).await;
    }
}


async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let filename = unsafe {
        format!("{}{}", WEB_ROOT, filename)
    };
    let contents = std::fs::read_to_string(filename.to_string()).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}