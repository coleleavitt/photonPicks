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
    #[serde(default)]
    pub dev_holding_perc: Option<f64>,
    pub dex_i: i64,
    #[serde(default)]
    pub fdv: f64,
    pub holders_count: i64,
    pub name: String,
    #[serde(default)]
    pub price_usd: Option<f64>,
    pub symbol: String,
    pub volume: f64,
    #[serde(default)]
    pub socials: Option<Socials>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAudit {
    pub freeze_authority: bool,
    #[serde(with = "string_or_float")]
    pub lp_burned_perc: f64,
    pub mint_authority: bool,
    #[serde(with = "string_or_float")]
    pub top_holders_perc: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Liquidity {
    #[serde(with = "string_or_float")]
    pub quote: f64,
    pub usd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Socials {
    #[serde(default)]
    pub medium: Option<String>,
    #[serde(default)]
    pub reddit: Option<String>,
    #[serde(default)]
    pub telegram: Option<String>,
    #[serde(default)]
    pub twitter: Option<String>,
    #[serde(default)]
    pub website: Option<String>,
}

// Custom serializer/deserializer for handling both string and float values
mod string_or_float {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => f64::from_str(&s).map_err(serde::de::Error::custom),
            StringOrFloat::Float(f) => Ok(f),
        }
    }
}