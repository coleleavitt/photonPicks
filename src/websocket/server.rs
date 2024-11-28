use crate::errors::Result;
use crate::handle_connection;
use crate::models::TokenMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

const ADDR: &str = "127.0.0.1:8080";

pub async fn start_websocket_server(token_map: Arc<RwLock<TokenMap>>) -> Result<()> {
    let listener = TcpListener::bind(ADDR).await?;
    println!("WebSocket server listening on ws://{ADDR}");

    while let Ok((stream, _)) = listener.accept().await {
        let token_map = Arc::clone(&token_map);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, token_map).await {
                eprintln!("Error handling connection: {e}");
            }
        });
    }

    Ok(())
}
