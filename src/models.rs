use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebSocketMessage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub data: Vec<TokenData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenData {
    pub attributes: TokenAttributes,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokenAttributes {
    Simple {
        action: String,
        #[serde(rename = "imgUrl")]
        img_url: String,
    },
    Token {
        address: String,
        audit: TokenAudit,
        #[serde(rename = "buys_count")]
        buys_count: i64,
        #[serde(rename = "created_timestamp")]
        created_timestamp: i64,
        #[serde(rename = "cur_liq")]
        cur_liq: Liquidity,
        #[serde(rename = "dev_holding_perc")]
        dev_holding_perc: Option<f64>,
        #[serde(rename = "dex_i")]
        dex_i: i64,
        fdv: f64,
        from_meme_dex: Option<Value>,
        from_moonshot: bool,
        from_pump: bool,
        #[serde(rename = "holders_count")]
        holders_count: i64,
        ignored: bool,
        #[serde(rename = "imgUrl")]
        img_url: Option<String>,
        #[serde(rename = "init_liq")]
        init_liq: InitialLiquidity,
        name: String,
        #[serde(rename = "open_timestamp")]
        open_timestamp: i64,
        #[serde(rename = "pooled_sol")]
        pooled_sol: f64,
        #[serde(rename = "price_usd")]
        price_usd: f64,
        #[serde(rename = "pump_migrated")]
        pump_migrated: Option<bool>,
        #[serde(rename = "pump_progress")]
        pump_progress: Option<i64>,
        #[serde(rename = "sells_count")]
        sells_count: i64,
        #[serde(rename = "snipers_count")]
        snipers_count: Option<i64>,
        socials: Option<Socials>,
        symbol: String,
        token_address: String,
        volume: f64,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenAudit {
    #[serde(rename = "freeze_authority")]
    pub freeze_authority: Option<bool>,
    #[serde(rename = "lp_burned_perc")]
    pub lp_burned_perc: i64,
    #[serde(rename = "mint_authority")]
    pub mint_authority: Option<bool>,
    #[serde(rename = "top_holders_perc")]
    pub top_holders_perc: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Liquidity {
    pub quote: f64,
    pub usd: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitialLiquidity {
    #[serde(rename = "lp_amount")]
    pub lp_amount: Option<i64>,
    #[serde(rename = "open_timestamp")]
    pub open_timestamp: Option<i64>,
    pub percentage: Option<f64>,
    pub quote: Option<f64>,
    pub timestamp: Option<i64>,
    pub token: Option<f64>,
    pub usd: Value,
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Socials {
    pub medium: Value,
    pub reddit: Value,
    pub telegram: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
}
