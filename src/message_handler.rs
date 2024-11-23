use std::fs::OpenOptions;
use std::io::Write;
use crate::models::*;

pub fn handle_token_message(message: &str) -> serde_json::Result<()> {
    let token_data: WebSocketMessage = serde_json::from_str(message)?;

    if let Ok(mut file) = OpenOptions::new()
        .append(true)
        .create(true)
        .open("token_messages.txt")
    {
        for entry in token_data.data {
            let log_message = match entry.attributes {
                TokenAttributes::Simple { action, img_url } => {
                    format!(
                        "Simple Update - Action: {}, Image: {}\n",
                        action, img_url
                    )
                },
                TokenAttributes::Token { name, symbol, price_usd, volume, holders_count, .. } => {
                    format!(
                        "Token Update - Name: {}, Symbol: {}, Price: ${}, Volume: ${:.2}, Holders: {}\n",
                        name, symbol, price_usd, volume, holders_count
                    )
                }
            };

            if let Err(e) = write!(file, "{}", log_message) {
                eprintln!("Error writing to file: {}", e);
            }
        }
    }
    Ok(())
}

pub fn handle_token_message_safe(message: &str) {
    if let Err(e) = std::panic::catch_unwind(|| {
        if let Err(e) = handle_token_message(message) {
            eprintln!("Error deserializing message: {}", e);
        }
    }) {
        eprintln!("Panic while handling token message: {:?}", e);
    }
}
