use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub attributes: Attributes,
    pub id: Box<str>,
    #[serde(rename = "type")]
    pub data_type: Box<str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl TokenData {
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn to_json_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn validate(&self) -> Result<()> {
        if self.id.is_empty() {
            return Err(WebSocketError::TokenParse("Empty token ID".into()));
        }
        if self.data_type.is_empty() {
            return Err(WebSocketError::TokenParse("Empty token type".into()));
        }
        Ok(())
    }
}

impl Attributes {
    pub fn get_display_name(&self) -> Cow<str> {
        self.name.as_deref()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Borrowed("Unknown Token"))
    }

    pub fn get_symbol_or_default(&self) -> Cow<str> {
        self.symbol.as_deref()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Borrowed("???"))
    }

    pub fn has_valid_price(&self) -> bool {
        self.price_usd.map_or(false, |p| p > 0.0)
    }

    pub fn has_valid_volume(&self) -> bool {
        self.volume.map_or(false, |v| v >= 0.0)
    }

    pub fn has_valid_holders(&self) -> bool {
        self.holders_count.map_or(false, |h| h > 0)
    }
}

impl Audit {
    pub fn is_safe(&self) -> bool {
        !self.freeze_authority &&
            !self.mint_authority &&
            self.lp_burned_perc >= 95 &&
            self.top_holders_perc <= 15.0
    }

    pub fn get_risk_score(&self) -> u8 {
        let mut score = 0;
        if self.freeze_authority { score += 2; }
        if self.mint_authority { score += 3; }
        if self.lp_burned_perc < 95 { score += 2; }
        if self.top_holders_perc > 15.0 { score += 1; }
        score
    }
}

impl InitialLiquidity {
    pub fn is_valid(&self) -> bool {
        self.quote.is_some() &&
            self.token.is_some() &&
            self.usd.is_some() &&
            !self.pending.unwrap_or(true)
    }

    pub fn get_total_value_usd(&self) -> Option<f64> {
        self.usd
    }
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

        let result = TokenData::from_json(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_audit_risk_score() {
        let audit = Audit {
            freeze_authority: true,
            mint_authority: true,
            lp_burned_perc: 90,
            top_holders_perc: 20.0,
        };
        assert_eq!(audit.get_risk_score(), 8);
        assert!(!audit.is_safe());
    }

    #[test]
    fn test_attributes_display() {
        let attrs = Attributes {
            name: Some("Test Token".into()),
            symbol: Some("TEST".into()),
            price_usd: Some(1.0),
            volume: Some(1000.0),
            holders_count: Some(100),
            ..Default::default()
        };

        assert_eq!(attrs.get_display_name(), "Test Token");
        assert_eq!(attrs.get_symbol_or_default(), "TEST");
        assert!(attrs.has_valid_price());
        assert!(attrs.has_valid_volume());
        assert!(attrs.has_valid_holders());
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            address: None,
            audit: None,
            buys_count: None,
            created_timestamp: None,
            cur_liq: None,
            dev_holding_perc: None,
            dex_i: None,
            fdv: None,
            from_meme_dex: None,
            from_moonshot: None,
            from_pump: None,
            holders_count: None,
            ignored: None,
            img_url: None,
            init_liq: None,
            name: None,
            open_timestamp: None,
            pooled_sol: None,
            price_usd: None,
            pump_migrated: None,
            pump_progress: None,
            sells_count: None,
            snipers_count: None,
            socials: None,
            symbol: None,
            token_address: None,
            volume: None,
            action: None,
        }
    }
}
