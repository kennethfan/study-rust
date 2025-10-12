use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        // 创建到服务器的连接
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, resp } => {
                    let result = client.get(&key).await;
                    let _ = resp.send(result);
                }
                Set { key, val, resp } => {
                    let result = client.set(&key, val).await;
                    let _ = resp.send(result);
                }
            }
        }
    });

    // 生成两个任务，一个用于获取 key，一个用于设置 key
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        };

        // 发送 GET 请求
        tx.send(cmd).await.unwrap();

        // 等待回复
        let result = resp_rx.await.unwrap();
        println!("GOT = {:?}", result);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };

        // 发送 SET 请求
        tx2.send(cmd).await.unwrap();
        // 等待回复
        let result = resp_rx.await.unwrap();
        println!("GOT = {:?}", result);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
