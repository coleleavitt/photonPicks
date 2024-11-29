use crate::errors::{Result, WebSocketError};
use crate::models::{TokenData, TokenMap};
use futures_util::{StreamExt, TryStreamExt};
use serde_json::Value;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

pub struct WebSocketServer {
    addr: String,
    token_manager: Arc<RwLock<TokenMap>>,
}

impl WebSocketServer {
    pub fn new(addr: String, token_manager: Arc<RwLock<TokenMap>>) -> Self {
        Self {
            addr,
            token_manager,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        println!("WebSocket server listening on ws://{}", self.addr);

        while let Ok((stream, _)) = listener.accept().await {
            let token_manager = Arc::clone(&self.token_manager);
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, token_manager).await {
                    eprintln!("Connection error: {e}");
                }
            });
        }
        Ok(())
    }

    async fn handle_connection(
        stream: TcpStream,
        token_manager: Arc<RwLock<TokenMap>>,
    ) -> Result<()> {
        let ws_stream = accept_async(stream).await?;
        let (_write, read) = ws_stream.split();

        read.try_for_each(|msg| {
            let token_manager = Arc::clone(&token_manager);
            async move {
                match msg {
                    Message::Text(text) => {
                        if let Err(e) = Self::process_message(&text, &token_manager).await {
                            eprintln!("Message processing error: {e}");
                        }
                    }
                    Message::Close(_) => return Ok(()),
                    _ => {}
                }
                Ok(())
            }
        })
        .await?;

        Ok(())
    }

    async fn process_message(text: &str, token_manager: &Arc<RwLock<TokenMap>>) -> Result<()> {
        let json: Value = serde_json::from_str(text)?;

        if let Some(tokens) = json.get("tokens") {
            Self::update_token_map(tokens, token_manager).await?;
        }
        Ok(())
    }

    pub async fn update_token_map(
        tokens: &Value,
        token_manager: &Arc<RwLock<TokenMap>>,
    ) -> Result<()> {
        let tokens_array = tokens
            .as_array()
            .ok_or_else(|| WebSocketError::TokenParse("Expected array".into()))?;

        for token_value in tokens_array {
            let token: TokenData = serde_json::from_value(token_value.clone())?;
            token_manager.write().await.insert(token.id.clone(), token);
        }

        Ok(())
    }
}
