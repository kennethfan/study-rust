mod download;
mod worker;
mod progress;

use std::sync::mpsc;
use std::thread;

fn main() {
    // 模拟几个要下载的文件
    let urls = vec![
        "https://www.rust-lang.org/logos/rust-logo-512x512.png",
        "https://doc.rust-lang.org/book/img/trpl20-cover.jpg",
    ];

    let (tx, rx) = mpsc::channel();

    // 为每个下载任务开一个线程
    for url in urls {
        let tx = tx.clone();
        let u = url.to_string();
        thread::spawn(move || {
            if let Err(e) = download::blocking_download(&u) {
                tx.send(format!("❌ {u} failed: {e}")).unwrap();
            } else {
                tx.send(format!("✅ {u} done")).unwrap();
            }
        });
    }

    drop(tx); // 关闭发送端

    // 主线程收集进度
    for msg in rx {
        println!("{}", msg);
    }
}

