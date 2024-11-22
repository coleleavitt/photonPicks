// src/main.rs

mod message_handler;
mod models;
mod token_analysis;
mod websocket;

#[tokio::main]
/// The main entry point of the application.
///
/// This function sets up a WebSocket server that listens for incoming connections
/// on the specified address. For each accepted connection, it spawns a new task
/// to handle the connection.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Returns an empty result on success,
///   or an error if the server setup or connection handling fails.
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