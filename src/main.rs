// src/main.rs
mod message_handler;
mod models;
mod websocket;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    println!("WebSocket server listening on ws://{}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        println!("New client connected: {}", addr);
        tokio::spawn(websocket::handle_connection(stream));
    }
}
