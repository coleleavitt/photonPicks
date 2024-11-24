use crate::error::Result;
use crate::models::Root;
use serde_json::from_str;
use std::fs::OpenOptions;
use std::io::Write;

/// Processes a token message by deserializing it and writing the names of the data objects to a file.
///
/// # Arguments
///
/// * `message` - A string slice that holds the JSON message to be processed.
///
/// # Returns
///
/// * `Result<()>` - Result indicating success or failure of the file operations.
pub fn process_token_message(message: &str) -> Result<()> {
    let root: Root = from_str(message)?;

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("pooled_sol_messages.txt")?;

    for token_data in root.data {
        if let (Some(name), Some(pooled_sol), Some(lp_burned_perc)) = (
            token_data.attributes.name.as_ref(),
            token_data.attributes.pooled_sol,
            token_data.attributes.audit.as_ref().map(|a| a.lp_burned_perc),
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
        if let Err(e) = process_token_message(message) {
            eprintln!("Error writing message to file: {}", e);
        }
    }) {
        eprintln!("Panic while handling token message: {:?}", e);
    }
}