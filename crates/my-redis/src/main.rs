use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

const REDIS_HOST: &str = "REDIS_HOST";
const REDIS_PORT: &str = "REDIS_PORT";

#[tokio::main]
async fn main() {
    let mut addr = std::env::var(REDIS_HOST.to_string()).unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var(REDIS_PORT.to_string()).unwrap_or_else(|_| "6379".to_string());
    // 建立与mini-redis服务器的连接
    addr.push_str(":");
    addr.push_str(&port);

    let listener = TcpListener::bind(&addr).await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Got {:?}", frame);


        // 回复一个错误
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}