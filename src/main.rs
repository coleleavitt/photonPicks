mod models;

use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use crate::models::{TokenData, TokenMap, WebSocketError, Result};
use futures_util::{StreamExt, TryStreamExt};

#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// Main entry point of the application.
/// Sets up a WebSocket server and listens for incoming connections.
#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("WebSocket server listening on ws://{}", addr);

    let token_map = Arc::new(RwLock::new(TokenMap::default()));
    let file_path = "tokens.txt";

    // Create the file if it does not exist
    if !std::path::Path::new(file_path).exists() {
        std::fs::File::create(file_path)?;
    }

    // Accept incoming connections in a loop
    while let Ok((stream, _)) = listener.accept().await {
        let token_map = Arc::clone(&token_map);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, token_map).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
    Ok(())
}

/// Handles an individual WebSocket connection.
///
/// # Arguments
///
/// * `stream` - The TCP stream for the connection.
/// * `token_map` - A shared, thread-safe map of tokens.
async fn handle_connection(
    stream: tokio::net::TcpStream,
    token_map: Arc<RwLock<TokenMap>>
) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    let (_write, read) = ws_stream.split();

    // Process each message received on the WebSocket connection
    read.try_for_each(|msg| {
        let token_map = Arc::clone(&token_map);
        async move {
            match msg {
                Message::Text(text) => process_message(&text, &token_map).await.expect("Error processing message"),
                Message::Close(_) => return Ok(()),
                _ => {}
            }
            Ok(())
        }
    }).await?;

    Ok(())
}

/// Processes a received WebSocket message.
///
/// # Arguments
///
/// * `text` - The text message received.
/// * `token_map` - A shared, thread-safe map of tokens.
async fn process_message(
    text: &str,
    token_map: &Arc<RwLock<TokenMap>>
) -> Result<()> {
    let json: serde_json::Value = serde_json::from_str(text)?;
    if let Some(tokens) = json.get("tokens") {
        update_token_map(tokens, token_map).await?;
        print_token_details(tokens)?;
        append_to_file("tokens.txt", tokens)?;
    }
    Ok(())
}

/// Updates the token map with new tokens.
///
/// # Arguments
///
/// * `tokens` - The JSON value containing the tokens.
/// * `token_map` - A shared, thread-safe map of tokens.
async fn update_token_map(
    tokens: &serde_json::Value,
    token_map: &Arc<RwLock<TokenMap>>
) -> Result<()> {
    let token_array = tokens.as_array()
        .ok_or_else(|| WebSocketError::TokenParse("Expected array".into()))?;

    let mut map = token_map.write().await;
    for token_value in token_array {
        let token: TokenData = serde_json::from_value(token_value.clone())?;
        map.insert(token.id.clone(), token);
    }
    Ok(())
}

/// Prints the details of the tokens to the console.
///
/// # Arguments
///
/// * `tokens` - The JSON value containing the tokens.
fn print_token_details(tokens: &serde_json::Value) -> Result<()> {
    let token_array = tokens.as_array()
        .ok_or_else(|| WebSocketError::TokenParse("Expected array".into()))?;

    for token_value in token_array {
        let token: TokenData = serde_json::from_value(token_value.clone())?;
        if let Some(name) = &token.attributes.name {
            print_single_token(&token, name)?;
        }
    }
    Ok(())
}

/// Prints the details of a single token to the console.
///
/// # Arguments
///
/// * `token` - The token data.
/// * `name` - The name of the token.
fn print_single_token(token: &TokenData, name: &str) -> Result<()> {
    let attrs = &token.attributes;
    println!("## Token Details");
    println!("Name: {}", name);

    if let Some(addr) = &attrs.token_address {
        println!("Address: {}", addr);
    }
    if let Some(symbol) = &attrs.symbol {
        println!("Symbol: {}", symbol);
    }
    if let Some(price) = attrs.price_usd {
        println!("Price: ${:.8}", price);
    }
    if let Some(volume) = attrs.volume {
        println!("Volume: ${:.2}", volume);
    }
    if let Some(holders) = attrs.holders_count {
        println!("Holders: {}", holders);
    }
    println!("---");
    Ok(())
}

/// Appends new tokens to a file.
///
/// # Arguments
///
/// * `file_path` - The path to the file.
/// * `new_tokens` - The JSON value containing the new tokens.
fn append_to_file(file_path: &str, new_tokens: &serde_json::Value) -> Result<()> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;

    let mut writer = BufWriter::new(file);

    if let serde_json::Value::Array(new_arr) = new_tokens {
        for token in new_arr {
            writeln!(writer, "{},", token)?;
        }
    }

    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::RwLock;
    use std::sync::Arc;
    use serde_json::json;
    use crate::models::Attributes;

    #[tokio::test]
    async fn test_process_message() {
        let token_map = Arc::new(RwLock::new(TokenMap::default()));
        let text = r#"{"tokens": [{"id": "123", "type": "token", "attributes": {"name": "TestToken"}}]}"#;

        let result = process_message(text, &token_map).await;
        assert!(result.is_ok());

        let map = token_map.read().await;
        assert!(map.contains_key("123"));
    }

    #[tokio::test]
    async fn test_update_token_map() {
        let token_map = Arc::new(RwLock::new(TokenMap::default()));
        let tokens = json!({
            "tokens": [{
                "id": "123",
                "type": "token",
                "attributes": {
                    "name": "TestToken",
                    "symbol": "TEST"
                }
            }]
        });

        let result = update_token_map(&tokens["tokens"], &token_map).await;
        assert!(result.is_ok());

        let map = token_map.read().await;
        assert!(map.contains_key("123"));
    }

    #[test]
    fn test_print_token_details() {
        let tokens = json!([{
            "id": "123",
            "type": "token",
            "attributes": {
                "name": "TestToken",
                "symbol": "TEST",
                "price_usd": 1.23,
                "volume": 1000000.0,
                "holders_count": 1000
            }
        }]);

        let result = print_token_details(&tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_single_token() {
        let token = TokenData {
            id: "123".into(),
            data_type: "token".into(),
            attributes: Attributes {
                name: Some("TestToken".into()),
                symbol: Some("TEST".into()),
                price_usd: Some(1.23),
                volume: Some(1000000.0),
                holders_count: Some(1000),
                ..Default::default()
            }
        };

        let result = print_single_token(&token, "TestToken");
        assert!(result.is_ok());
    }

    #[test]
    fn test_append_to_file() {
        use std::fs;
        use tempfile::NamedTempFile;

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let tokens = json!([{
            "id": "123",
            "type": "token",
            "attributes": {
                "name": "TestToken"
            }
        }]);

        let result = append_to_file(path, &tokens);
        assert!(result.is_ok());

        let contents = fs::read_to_string(path).unwrap();
        assert!(contents.contains("TestToken"));
    }

    #[tokio::test]
    async fn test_handle_connection() {
        use tokio::net::TcpListener;
        use futures_util::SinkExt;
        use tokio_tungstenite::connect_async;

        let token_map = Arc::new(RwLock::new(TokenMap::default()));
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let server = tokio::spawn({
            let token_map = Arc::clone(&token_map);
            async move {
                let (stream, _) = listener.accept().await.unwrap();
                handle_connection(stream, token_map).await.unwrap();
            }
        });

        let (mut ws_stream, _) = connect_async(format!("ws://{}", addr))
            .await
            .unwrap();

        let msg = json!({
            "tokens": [{
                "id": "123",
                "type": "token",
                "attributes": { "name": "TestToken" }
            }]
        });

        ws_stream.send(Message::Text(msg.to_string())).await.unwrap();
        ws_stream.close(None).await.unwrap();

        server.await.unwrap();
    }
}
