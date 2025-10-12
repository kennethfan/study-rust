use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // 在这里拷贝数据
            let mut buf = [0; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        // Copy the data back to socket
                        // 将数据拷贝回 socket 中
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // 非预期错误，由于我们这里无需再做什么，因此直接停止处理
                            return;
                        }
                    }
                    // 非预期错误，由于我们无需再做什么，因此直接停止处理
                    Err(_) => return,
                }
            }
        });
    }
}