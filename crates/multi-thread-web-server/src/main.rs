use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{env, thread};
use multi_thread_web_server::ThreadPool;

static mut WEB_ROOT: String = String::new();

fn main() {
    unsafe {
        WEB_ROOT = env::var("WEB_ROOT".to_string()).unwrap_or("".to_string());
    }

    let pool = ThreadPool::new(4);

    let listener = TcpListener::bind("127.0.0.1:6666").unwrap();
    println!("listening!");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
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