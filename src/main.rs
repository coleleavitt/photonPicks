// src/main.rs
mod message_handler;
mod models;
mod token_analysis;
mod websocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Your existing websocket server setup code
    let addr = "127.0.0.1:8080";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(websocket::handle_connection(stream));
    }

    Ok(())
}
