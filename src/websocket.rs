// src/websocket.rs
use crate::message_handler::handle_token_message;
use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{accept_async, WebSocketStream};

/// Handles an incoming TCP connection and upgrades it to a WebSocket connection.
///
/// # Arguments
///
/// * `stream` - A `TcpStream` representing the incoming TCP connection.
pub async fn handle_connection(stream: TcpStream) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Failed to accept websocket connection: {}", e);
            return;
        }
    };

    process_websocket(ws_stream).await;
}

/// Processes the WebSocket stream by reading messages and handling them.
///
/// # Arguments
///
/// * `ws_stream` - A `WebSocketStream<TcpStream>` representing the WebSocket connection.
async fn process_websocket(ws_stream: WebSocketStream<TcpStream>) {
    let (_write, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                if let Message::Text(text) = msg {
                    handle_token_message(&text);
                }
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }
}