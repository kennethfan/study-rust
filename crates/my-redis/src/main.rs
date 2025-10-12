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
        // process(socket).await;
        // 为每一条连接都生成一个新的任务，
        // `socket` 的所有权将被移动到新的任务中，并在那里进行处理
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // 使用 hashmap 来存储 redis 的数据
    let mut db = HashMap::new();

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Got {:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}