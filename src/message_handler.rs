// src/message_handler.rs
use crate::models::TokenResponse;

pub fn handle_token_message(message: &str) {
    match serde_json::from_str::<TokenResponse>(message) {
        Ok(token_response) => {
            println!("Received token data:");
            for token in token_response.data {
                println!(
                    "Token: {} ({})",
                    token.attributes.name, token.attributes.symbol
                );
                println!("Price: ${:?}", token.attributes.price_usd);
                println!("Volume: ${:.2}", token.attributes.volume);
                println!("Holders: {}", token.attributes.holders_count);
                println!("---");
            }
        }
        Err(e) => {
            eprintln!("Failed to parse message: {}", e);
        }
    }
}
