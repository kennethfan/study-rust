use mini_redis::{client, Result};

const REDIS_HOST: &str = "REDIS_HOST";
const REDIS_PORT: &str = "REDIS_PORT";

#[tokio::main]
async fn main() -> Result<()> {
    let mut addr = std::env::var(REDIS_HOST.to_string()).unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var(REDIS_PORT.to_string()).unwrap_or_else(|_| "6379".to_string());
    // 建立与mini-redis服务器的连接
    addr.push_str(":");
    addr.push_str(&port);
    let mut client = client::connect(addr).await?;

    // 设置 key: "hello" 和 值: "world"
    client.set("hello", "world".into()).await?;

    // 获取"key=hello"的值
    let result = client.get("hello").await?;

    println!("从服务器端获取到结果={:?}", result);

    Ok(())
}