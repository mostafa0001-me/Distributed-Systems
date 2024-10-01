use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    let msg = "Leader candidate: Node 1";
    socket.write_all(msg.as_bytes()).await.unwrap();

    let mut buf = [0u8; 1024];
    let n = socket.read(&mut buf).await.unwrap();

    println!("Server response: {}", String::from_utf8_lossy(&buf[..n]));
}

