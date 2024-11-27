use serde::{Deserialize, Serialize};
use rustc_hash::FxHashMap;

#[derive(Debug, thiserror::Error)]
pub enum WebSocketError {
    #[error("Failed to parse token data: {0}")]
    TokenParse(String),
    #[error("WebSocket error: {0}")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, WebSocketError>;
pub type TokenMap = FxHashMap<Box<str>, TokenData>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub attributes: Attributes,
    pub id: Box<str>,
    #[serde(rename = "type")]
    pub data_type: Box<str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Attributes {
    pub address: Option<Box<str>>,
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
    pub img_url: Option<Box<str>>,
    pub init_liq: Option<InitialLiquidity>,
    pub name: Option<Box<str>>,
    pub open_timestamp: Option<i64>,
    pub pooled_sol: Option<f64>,
    pub price_usd: Option<f64>,
    pub pump_migrated: Option<bool>,
    pub pump_progress: Option<u8>,
    pub sells_count: Option<u64>,
    pub snipers_count: Option<u32>,
    pub socials: Option<Socials>,
    pub symbol: Option<Box<str>>,
    #[serde(rename = "tokenAddress")]
    pub token_address: Option<Box<str>>,
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
    pub telegram: Option<Box<str>>,
    pub twitter: Option<Box<str>>,
    pub website: Option<Box<str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Overwrite,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let json = r#"{
            "attributes": {
                "address": "test",
                "audit": {
                    "freeze_authority": false,
                    "lp_burned_perc": 100,
                    "mint_authority": false,
                    "top_holders_perc": 10.5
                }
            },
            "id": "123",
            "type": "token"
        }"#;

        let result = serde_json::from_str::<TokenData>(json);
        assert!(result.is_ok());
    }
}
