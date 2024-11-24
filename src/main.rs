mod message_handler;
mod models;
mod websocket;
mod error;

use crate::error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let addr = "127.0.0.1:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.map_err(|e| AppError::Connection(e.to_string()))?;
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(e) = websocket::handle_connection(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }

    Ok(())
}