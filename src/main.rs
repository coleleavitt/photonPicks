//! Main module for the WebSocket server.

mod models;
mod math;
mod errors;

use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

use crate::models::{TokenData, TokenMap};
use crate::math::{generate_wallet_holdings, collect_recent_trades};
use crate::errors::{Result, WebSocketError};

use futures_util::{StreamExt, TryStreamExt};
const ADDR: &str = "127.0.0.1:8080";
const FILE_PATH: &str = "tokens.txt";



#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind(ADDR).await?;
    println!("WebSocket server listening on ws://{ADDR}");

    let token_map = Arc::new(RwLock::new(TokenMap::default()));

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

async fn handle_connection(
    stream: tokio::net::TcpStream,
    token_map: Arc<RwLock<TokenMap>>,
) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    let (_write, read) = ws_stream.split();

    read.try_for_each(|msg| {
        let token_map = Arc::clone(&token_map);
        return async move {
            match msg {
                Message::Text(text) => {
                    if let Err(e) = process_message(&text, &token_map).await {
                        eprintln!("Error processing message: {e}");
                    }
                }
                Message::Close(_) => return Ok(()),
                _ => {}
            }
            return Ok(());
        };
    })
        .await?;

    Ok(())
}

async fn process_message(text: &str, token_map: &Arc<RwLock<TokenMap>>) -> Result<()> {
    let json: serde_json::Value = serde_json::from_str(text)?;
    if let Some(tokens) = json.get("tokens") {
        update_token_map(tokens, token_map).await?;
        print_token_details(tokens)?;
        // append_to_file("tokens.txt", tokens)?;
    }
    Ok(())
}

async fn update_token_map(
    tokens: &serde_json::Value,
    token_map: &Arc<RwLock<TokenMap>>,
) -> Result<()> {
    tokens.as_array()
        .ok_or_else(|| return WebSocketError::TokenParse("Expected array".into()))?;

    for token_value in tokens.as_array().unwrap() {
        let token: TokenData = serde_json::from_value(token_value.clone())?;
        token_map.write().await.insert(token.id.clone(), token);
    }

    Ok(())
}

fn print_token_details(tokens: &serde_json::Value) -> Result<()> {
    tokens.as_array()
        .ok_or_else(|| return WebSocketError::TokenParse("Expected array".into()))?;

    for token_value in tokens.as_array().unwrap() {
        let token: TokenData = serde_json::from_value(token_value.clone())?;
        if let Some(name) = &token.attributes.name {
            print_single_token(&token, name);
        }
    }

    Ok(())
}

fn print_single_token(token: &TokenData, name: &str) {
    println!("## Token Details");
    println!("Name: {name}");
    
    if (token.attributes.fdv.is_some()) && token.attributes.fdv > Option::from(50000.0) {
        if let Some(addr) = &token.attributes.token_address { println!("Address: {addr}"); }
        if let Some(symbol) = &token.attributes.symbol { println!("Symbol: {symbol}"); }
        if let Some(price) = token.attributes.price_usd { println!("Price: ${price:.8}"); }
        if let Some(volume) = token.attributes.volume { println!("Volume: ${volume:.2}"); }
        if let Some(holders) = token.attributes.holders_count { println!("Holders: {holders}"); }
        if let Some(market_cap) = token.attributes.fdv { println!("Market Cap: ${market_cap:.2}"); }

    }
    let trades = collect_recent_trades(token);
    let wallet_holdings = generate_wallet_holdings(token);
    let hhi = token.calculate_adjusted_concentration(&wallet_holdings, &trades);
    let risk_level = token.get_concentration_risk(hhi);

    println!("Concentration Score: {:.4}", hhi);
    println!("Risk Level: {}", risk_level);
    println!("---\n---");
}


fn append_to_file(file_path: &str, new_tokens: &serde_json::Value) -> Result<()> {
    if !std::path::Path::new(FILE_PATH).exists() {
        std::fs::File::create(FILE_PATH)?;
    }
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;

    let mut writer = BufWriter::new(file);

    if let serde_json::Value::Array(new_arr) = new_tokens {
        for token in new_arr {
            writeln!(writer, "{token},")?;
        }
    }

    writer.flush()?;

    Ok(())
}
