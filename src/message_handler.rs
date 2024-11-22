use crate::models::TokenResponse;
use crate::token_analysis::{TokenFilter, TokenMetrics};

/// Handles a token message by parsing it and processing the tokens that meet the filter criteria.
///
/// # Arguments
///
/// * `message` - A string slice that holds the JSON message containing token data.
pub fn handle_token_message(message: &str) {
    match serde_json::from_str::<TokenResponse>(message) {
        Ok(token_response) => {
            let filter = TokenFilter::default();

            for token in token_response
                .data
                .iter()
                .filter(|t| filter.meets_criteria(t))
            {
                let metrics = TokenMetrics::from_token(token);
                print_token_metrics(&metrics);
            }
        }
        Err(e) => {
            eprintln!("Failed to parse message: {}", e);
            eprintln!("Response: {}", message);
        }
    }
}

/// Prints the metrics of a token to the console.
///
/// # Arguments
///
/// * `metrics` - A reference to `TokenMetrics` containing the metrics of the token.
fn print_token_metrics(metrics: &TokenMetrics) {
    println!("🚀 Trending Token Found:");
    println!("Name: {} ({})", metrics.name, metrics.symbol);
    println!("Price: ${:.8}", metrics.price_usd.unwrap_or_default());
    println!("Market Cap: ${:.2}", metrics.market_cap);
    println!("Holders: {}", metrics.holders);
    println!("Top Holders %: {:.2}%", metrics.top_holders_perc);
    println!("Volume: ${:.2}", metrics.volume);
    println!("Volume/MCap: {:.3}", metrics.volume_mcap_ratio);
    println!("Buy/Sell Ratio: {:.2}", metrics.buy_sell_ratio);
    println!("Age (hours): {:.2}", metrics.age_hours);
    println!("---");
}