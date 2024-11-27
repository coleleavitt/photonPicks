use crate::errors::Result;
use futures_util::StreamExt;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_tungstenite::{
    accept_async,
    tungstenite::protocol::Message,
};
use std::collections::HashMap;
use uuid::Uuid;
use crate::models::token::TokenProcessor;

pub struct ConnectionManager {
    connections: Arc<RwLock<HashMap<String, tokio::sync::mpsc::Sender<Message>>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn handle_connection(
        &self,
        stream: tokio::net::TcpStream,
        token_processor: &TokenProcessor,
    ) -> Result<()> {
        let ws_stream = accept_async(stream).await?;
        let (_, mut ws_receiver) = ws_stream.split();
        let conn_id = Uuid::new_v4().to_string();

        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str(&text) {
                        Ok(token) => {
                            if let Err(e) = token_processor.process_token(token).await {
                                tracing::error!("Error processing token: {}", e);
                            }
                        }
                        Err(e) => tracing::error!("Failed to parse JSON: {}", e),
                    }
                }
                Ok(Message::Close(_)) => break,
                Err(e) => {
                    tracing::error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        self.connections.write().await.remove(&conn_id);
        Ok(())
    }
}

impl Clone for ConnectionManager {
    fn clone(&self) -> Self {
        Self {
            connections: Arc::clone(&self.connections),
        }
    }
}