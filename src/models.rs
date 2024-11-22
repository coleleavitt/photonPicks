// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    #[serde(rename = "type")]
    pub message_type: String,
    pub data: Vec<TokenData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub attributes: TokenAttributes,
    pub id: String,
    #[serde(rename = "type")]
    pub data_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAttributes {
    pub address: String,
    pub audit: TokenAudit,
    pub buys_count: i64,
    pub created_timestamp: i64,
    pub cur_liq: Liquidity,
    pub dev_holding_perc: Option<f64>,
    pub dex_i: i64,
    pub fdv: f64,
    pub holders_count: i64,
    pub name: String,
    pub price_usd: Option<f64>,
    pub symbol: String,
    pub volume: f64,
    pub socials: Option<Socials>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAudit {
    pub freeze_authority: bool,
    pub lp_burned_perc: f64,
    pub mint_authority: bool,
    pub top_holders_perc: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Liquidity {
    pub quote: f64,
    pub usd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Socials {
    pub medium: Option<String>,
    pub reddit: Option<String>,
    pub telegram: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
}
