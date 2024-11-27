mod models;

use futures_util::{StreamExt, TryStreamExt};
use rustc_hash::FxHashMap;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use crate::models::{TokenData, WebSocketError, Result};

#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("WebSocket server listening on ws://{}", addr);

    let file_path = "tokens.txt";
    if !std::path::Path::new(file_path).exists() {
        std::fs::File::create(file_path)?;
    }

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    let (write, read) = ws_stream.split();

    read.try_for_each(|msg| async {
        match msg {
            Message::Text(text) => process_message(&text).await.expect("fuck"),
            Message::Close(_) => return Ok(()),
            _ => {}
        }
        Ok(())
    }).await?;

    Ok(())
}

async fn process_message(text: &str) -> Result<()> {
    let json: serde_json::Value = serde_json::from_str(text)?;
    if let Some(tokens) = json.get("tokens") {
        print_token_details(tokens)?;
    }
    Ok(())
}

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
