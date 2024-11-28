use crate::errors::{Result, WebSocketError};
use crate::models::{TokenData, TokenMap};
use futures_util::{StreamExt, TryStreamExt};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

pub async fn handle_connection(
    stream: tokio::net::TcpStream,
    token_map: Arc<RwLock<TokenMap>>,
) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    let (_write, read) = ws_stream.split();

    read.try_for_each(|msg| {
        let token_map = Arc::clone(&token_map);
        async move {
            match msg {
                Message::Text(text) => {
                    if let Err(e) = process_message(&text, &token_map).await {
                        eprintln!("Error processing message: {e}");
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

pub async fn process_message(text: &str, token_map: &Arc<RwLock<TokenMap>>) -> Result<()> {
    let json: serde_json::Value = serde_json::from_str(text)?;
    if let Some(tokens) = json.get("tokens") {
        update_token_map(tokens, token_map).await?;
    }
    Ok(())
}

pub async fn update_token_map(
    tokens: &serde_json::Value,
    token_map: &Arc<RwLock<TokenMap>>,
) -> Result<()> {
    let tokens_array = tokens
        .as_array()
        .ok_or_else(|| WebSocketError::TokenParse("Expected array".into()))?;

    for token_value in tokens_array {
        let token: TokenData = serde_json::from_value(token_value.clone())?;
        token_map.write().await.insert(token.id.clone(), token);
    }

    Ok(())
}
