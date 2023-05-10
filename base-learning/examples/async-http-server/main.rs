use std::{cmp::min, fs, time::Duration};

use async_std::io::Read;
use async_std::io::Write;
use async_std::prelude::*;
use async_std::{net::TcpListener, task};
use futures::StreamExt;

#[async_std::main]
async fn main() {
    // 使用异步的 async_std::net::TcpListener
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    // 如果只是调用异步函数 + await 并不能并发处理
    // 原因在于 listener.incoming 是一个阻塞的 iterator
    // for stream in listener.incoming() {
    // let stream = stream.unwrap();
    // Warning: This is not concurrent!
    // handle_connection_async(stream).await;
    // }
    // for_each_concurrent processes each connection concurrently, but on the same thread.
    listener
        .incoming()
        .for_each_concurrent(None, |tcpstream| async move {
            let tcpstream = tcpstream.unwrap();
            // 可以使用 async_std task 提供的 spawn 来多线程处理任务
            use async_std::task::spawn;
            spawn(handle_connection(tcpstream));
            // handle_connection_async(tcpstream).await;
        })
        .await;
}

// async fn handle_connection_async(mut stream: TcpStream) {
async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        // 这里的文件需要读取 project 根目录而不是 workspace 下的根目录的文件
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            "z_files/base_learning/async-http-server/hello.html",
        )
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            "z_files/base_learning/async-http-server/hello.html",
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND \r\n\r\n",
            "z_files/base_learning/async-http-server/404.html",
        )
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{status_line}{contents}");
    // write response back to the stream
    stream.write_all(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

// 单线程阻塞版
// fn handle_connection(mut stream: TcpStream) {

//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();

//     let get = b"GET / HTTP/1.1\r\n";

//     let (status_line, filename) = if buffer.starts_with(get) {
//         // 这里的文件需要读取 project 根目录而不是 workspace 下的根目录的文件
//         ("HTTP/1.1 200 OK\r\n\r\n", "z_files/base_learning/async-http-server/hello.html")
//     } else {
//         ("HTTP/1.1 404 NOT FOUND \r\n\r\n", "z_files/base_learning/async-http-server/404.html")
//     };

//     let contents = fs::read_to_string(filename).unwrap();

//     let response = format!("{status_line}{contents}");
//     // write response back to the stream
//     stream.write_all(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }

// 测试用的 Mock TcpStream
struct MockTcpStream {
    read_data: Vec<u8>,
    write_data: Vec<u8>,
}

impl Read for MockTcpStream {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        _: &mut task::Context<'_>,
        buf: &mut [u8],
    ) -> task::Poll<std::io::Result<usize>> {
        let size: usize = min(self.read_data.len(), buf.len());
        buf[..size].copy_from_slice(&self.read_data[..size]);
        futures::task::Poll::Ready(Ok(size))
    }
}

impl Write for MockTcpStream {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        _: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<std::io::Result<usize>> {
        self.write_data = Vec::from(buf);
        task::Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _: &mut task::Context<'_>,
    ) -> task::Poll<std::io::Result<()>> {
        task::Poll::Ready(Ok(()))
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        _: &mut task::Context<'_>,
    ) -> task::Poll<std::io::Result<()>> {
        task::Poll::Ready(Ok(()))
    }
}

impl Unpin for MockTcpStream {}

// 直接 run 会执行失败，debug 却没有问题？？
// run 的结果：thread 'test_handle_connection' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', base-learning/examples/async-http-server/main.rs:51:49
// run 执行的是 cargo test --package base-learning --example async-http-server -- test_handle_connection --exact --nocapture
// 而 debug 执行的是 /Users/zmlgirl/Documents/codes/github/zmlgirl/rust-learning/target/debug/examples/async_http_server-f93da5f1a747f102 test_handle_connection --exact --nocapture
#[async_std::test]
async fn test_handle_connection() {
    let input_bytes = b"GET / HTTP/1.1\r\n";
    let mut contents = vec![0u8; 1024];
    contents[..input_bytes.len()].clone_from_slice(input_bytes);
    let mut stream = MockTcpStream {
        read_data: contents,
        write_data: Vec::new(),
    };
    handle_connection(&mut stream).await;

    let expected_contents =
        fs::read_to_string("z_files/base_learning/async-http-server/hello.html").unwrap();
    let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
    assert!(stream.write_data.starts_with(expected_response.as_bytes()));
}
