use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        val: Bytes,
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
                Get { key } => {
                    client.get(&key).await;
                }
                Set { key, val } => {
                    client.set(&key, val).await;
                }
            }
        }
    });

    // 生成两个任务，一个用于获取 key，一个用于设置 key
    let t1 = tokio::spawn(async move {
        let cmd = Command::Get {
            key: "hello".to_string(),
        };

        tx.send(cmd).await.unwrap();
    });

    let t2 = tokio::spawn(async move {
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
        };

        tx2.send(cmd).await.unwrap();
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
