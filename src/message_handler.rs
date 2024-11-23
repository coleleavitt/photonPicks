// src/message_handler.rs
use crate::models::Root;
use serde_json::from_str;
use std::fs::OpenOptions;
use std::io::Write;

/// Handles a token message by deserializing it and writing the names of the data objects to a file.
///
/// # Arguments
///
/// * `message` - A string slice that holds the JSON message to be processed.
///
/// # Returns
///
/// * `std::io::Result<()>` - Result indicating success or failure of the file operations.
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
        if let (Some(name), Some(pooled_sol), Some(lp_burned_perc)) = (
            daum.attributes.name.as_ref(),
            daum.attributes.pooled_sol,
            daum.attributes.audit.as_ref().map(|a| a.lp_burned_perc),
        ) {
            if pooled_sol >= 2.0 && lp_burned_perc == 100 {
                writeln!(file, "{}", name)?;
            }
        }
    }

    Ok(())
}

/// Safely handles a token message by catching any panics that occur during processing.
///
/// # Arguments
///
/// * `message` - A string slice that holds the JSON message to be processed.
pub fn handle_token_message_safe(message: &str) {
    if let Err(e) = std::panic::catch_unwind(|| {
        if let Err(e) = handle_token_message(message) {
            eprintln!("Error writing message to file: {}", e);
        }
    }) {
        eprintln!("Panic while handling token message: {:?}", e);
    }
}
