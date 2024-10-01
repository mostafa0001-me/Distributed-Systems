use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;

type SharedState = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on port 8080");

    let state: SharedState = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let state = Arc::clone(&state);

        println!("New client connected: {}", addr);

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];

            let n = socket.read(&mut buf).await.unwrap();
            if n == 0 {
                return;
            }

            let request = String::from_utf8_lossy(&buf[..n]);
            println!("Received: {}", request);

            let mut state = state.lock().await;
            state.insert(addr.to_string(), request.to_string());

            socket.write_all(b"Message received").await.unwrap();
        });
    }
}

