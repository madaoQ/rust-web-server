use tokio::io::{AsyncReadExt, AsyncWriteExt}; // 引入异步 I/O 特性
use tokio::net::TcpListener; // 引入 Tokio 的 TcpListener
use std::fs::File; // 用于异步文件操作，但这里我们先用同步 fs::read_to_string
use std::io::ErrorKind; // 用于错误处理
use std::path::Path;

// 注意：handle_connection 函数需要变成 async
async fn handle_connection_async(mut stream: tokio::net::TcpStream) {
    let mut buffer = [0; 1024];
    // 使用 stream.read() 的异步版本
    let bytes_read = stream.read(&mut buffer).await.unwrap();

    let request_line = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Request: {}", request_line);

    let get = b"GET / HTTP/1.1\r\n";
    let get_hello = b"GET /hello HTTP/1.1\r\n";

    let (status_line, filename) = if request_line.starts_with("GET / HTTP/1.1") { // 这里因为request_line已经是字符串了，所以用starts_with
        ("HTTP/1.1 200 OK", "hello.html")
    } else if request_line.starts_with("GET /hello HTTP/1.1") {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // 文件读取仍然使用同步的 fs::read_to_string，因为异步文件 I/O 需要 tokio::fs，
    // 对于这个简单示例，同步读取在测试时影响不大，但在真实世界应用中会阻塞
    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        fs::read_to_string("404.html").unwrap_or_else(|_| {
            "<h1>Error: Page not found and 404.html missing!</h1>".to_string()
        })
    });

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // 使用 stream.write_all() 的异步版本
    stream.write_all(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}


#[tokio::main] // 标记 main 函数为 Tokio 异步入口点
async fn main() { // main 函数也需要是 async
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap(); // 绑定监听器也需要 await

    println!("Tokio async server listening on 127.0.0.1:7878");

    loop { // 异步服务器通常使用 loop 循环
        let (stream, _addr) = listener.accept().await.unwrap(); // 接受连接也需要 await

        // 为每个连接创建一个异步任务 (task)
        tokio::spawn(async move { // async move 闭包，将 stream 移动到 task 中
            handle_connection_async(stream).await; // 调用异步处理函数也需要 await
        });
    }
}