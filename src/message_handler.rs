use std::fs::OpenOptions;
use std::io::Write;
use serde_json::from_str;
use crate::models::Root;

pub fn handle_token_message(message: &str) -> std::io::Result<()> {
    let root: Root = match from_str(message) {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Failed to deserialize message: {}", e);
            return Ok(());
        }
    };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("token_messages.txt")?;

    for daum in root.data {
        if let Some(name) = daum.attributes.name {
            writeln!(file, "{}", name)?;
        }
    }

    Ok(())
}

pub fn handle_token_message_safe(message: &str) {
    if let Err(e) = std::panic::catch_unwind(|| {
        if let Err(e) = handle_token_message(message) {
            eprintln!("Error writing message to file: {}", e);
        }
    }) {
        eprintln!("Panic while handling token message: {:?}", e);
    }
}