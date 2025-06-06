use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs; // 用于读取文件

use std::thread;

// 定义一个简单的请求处理函数，用于解析和响应
fn handle_connection(mut stream: TcpStream) {
    // 1. 读取请求
    let mut buffer = [0; 1024]; // 缓冲区，用于存储传入数据
    stream.read(&mut buffer).unwrap();

    // 将缓冲区内容转换为字符串，方便查看和解析
    let request_line = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request_line); // 打印请求，方便调试

    // 2. 简单的请求解析（只处理 GET / 和 GET /hello）
    let get = b"GET / HTTP/1.1\r\n";
    let get_hello = b"GET /hello HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html") // 根路径返回 hello.html
    } else if buffer.starts_with(get_hello) {
        ("HTTP/1.1 200 OK", "hello.html") // /hello 也返回 hello.html，你可以改成不同的页面
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html") // 其他路径返回 404
    };

    // 3. 读取文件内容
    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        fs::read_to_string("404.html").unwrap_or_else(|_| {
            "<h1>Error: Page not found and 404.html missing!</h1>".to_string()
        })
    });

    // 4. 构建响应
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // 5. 发送响应
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Multi-threaded server listening on 127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // 为每个连接创建一个新线程
        thread::spawn(move || { // `move` 关键字将 `stream` 的所有权移动到新线程中
            handle_connection(stream);
        });
    }
}