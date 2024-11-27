use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::errors::{Result, WebSocketError};

#[derive(Debug, Clone)]
pub struct TokenProcessor {
    storage: Arc<RwLock<TokenStorage>>,
}

#[derive(Debug)]
struct TokenStorage {
    tokens: Vec<TokenData>,
}

impl TokenProcessor {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(TokenStorage { tokens: Vec::new() })),
        }
    }

    pub fn print_token_details(token: &TokenData) -> Result<()> {
        if let Some(name) = &token.attributes.name {
            println!("## Token Details");
            println!("Name: {}", name);

            if let Some(addr) = &token.attributes.token_address {
                println!("Address: {}", addr);
            }

            if let Some(symbol) = &token.attributes.symbol {
                println!("Symbol: {}", symbol);
            }

            if let Some(price) = token.attributes.price_usd {
                println!("Price: ${:.8}", price);
            }

            if let Some(volume) = token.attributes.volume {
                println!("Volume: ${:.2}", volume);
            }

            if let Some(holders) = token.attributes.holders_count {
                println!("Holders: {}", holders);
            }

            println!("---");
        }
        Ok(())
    }

    pub async fn process_token(&self, token: TokenData) -> Result<()> {
        if !self.validate_token(&token) {
            return Err(WebSocketError::ValidationError("Invalid token data".to_string()));
        }

        // Print token details before storing
        Self::print_token_details(&token)?;

        let mut storage = self.storage.write().await;
        storage.tokens.push(token);
        Ok(())
    }

    fn validate_token(&self, token: &TokenData) -> bool {
        token.attributes.name.is_some() && token.attributes.token_address.is_some()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub attributes: Option<Attributes>,
    pub id: String,
    #[serde(rename = "type")]
    pub data_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attributes {
    pub address: Option<String>,
    pub audit: Option<Audit>,
    pub buys_count: Option<u64>,
    pub created_timestamp: Option<i64>,
    pub cur_liq: Option<Liquidity>,
    pub dev_holding_perc: Option<f64>,
    pub dex_i: Option<u8>,
    pub fdv: Option<f64>,
    #[serde(rename = "fromMemeDex")]
    pub from_meme_dex: Option<u8>,
    #[serde(rename = "fromMoonshot")]
    pub from_moonshot: Option<bool>,
    #[serde(rename = "fromPump")]
    pub from_pump: Option<bool>,
    pub holders_count: Option<u32>,
    pub ignored: Option<bool>,
    #[serde(rename = "imgUrl")]
    pub img_url: Option<String>,
    pub init_liq: Option<InitialLiquidity>,
    pub name: Option<String>,
    pub open_timestamp: Option<i64>,
    pub pooled_sol: Option<f64>,
    pub price_usd: Option<f64>,
    pub pump_migrated: Option<bool>,
    pub pump_progress: Option<u8>,
    pub sells_count: Option<u64>,
    pub snipers_count: Option<u32>,
    pub socials: Option<Socials>,
    pub symbol: Option<String>,
    #[serde(rename = "tokenAddress")]
    pub token_address: Option<String>,
    pub volume: Option<f64>,
    pub action: Option<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audit {
    pub freeze_authority: bool,
    pub lp_burned_perc: u8,
    pub mint_authority: bool,
    pub top_holders_perc: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Liquidity {
    pub quote: f64,
    pub usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialLiquidity {
    pub quote: Option<f64>,
    pub timestamp: Option<i64>,
    pub token: Option<f64>,
    pub usd: Option<f64>,
    pub lp_amount: Option<u64>,
    pub open_timestamp: Option<i64>,
    pub percentage: Option<f64>,
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Socials {
    pub medium: Option<serde_json::Value>,
    pub reddit: Option<serde_json::Value>,
    pub telegram: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Overwrite,
}
