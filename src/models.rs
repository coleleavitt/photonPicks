// src/models.rs

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

/// Represents the root structure of the JSON data.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    /// The type of the root object.
    #[serde(rename = "type")]
    pub type_field: String,
    /// A list of data objects.
    pub data: Vec<Daum>,
}

/// Represents a data object.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    /// The attributes of the data object.
    pub attributes: Attributes,
    /// The ID of the data object.
    pub id: String,
    /// The type of the data object.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// Represents the attributes of a data object.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    /// The address associated with the data object.
    pub address: Option<String>,
    /// The audit information of the data object.
    pub audit: Option<Audit>,
    /// The count of buys.
    #[serde(rename = "buys_count")]
    pub buys_count: Option<i64>,
    /// The timestamp when the object was created.
    #[serde(rename = "created_timestamp")]
    pub created_timestamp: Option<i64>,
    /// The current liquidity information.
    #[serde(rename = "cur_liq")]
    pub cur_liq: Option<CurLiq>,
    /// The percentage of developer holdings.
    #[serde(rename = "dev_holding_perc")]
    pub dev_holding_perc: Option<f64>,
    /// The DEX index.
    #[serde(rename = "dex_i")]
    pub dex_i: Option<i64>,
    /// The fully diluted valuation.
    pub fdv: Option<f64>,
    /// Indicates if the object is from meme DEX.
    pub from_meme_dex: Option<i64>,
    /// Indicates if the object is from moonshot.
    pub from_moonshot: Option<bool>,
    /// Indicates if the object is from pump.
    pub from_pump: Option<bool>,
    /// Duplicate field for meme DEX.
    #[serde(rename = "from_meme_dex")]
    pub from_meme_dex2: Option<i64>,
    /// The count of holders.
    #[serde(rename = "holders_count")]
    pub holders_count: Option<i64>,
    /// Indicates if the object is ignored.
    pub ignored: Option<bool>,
    /// The URL of the image.
    pub img_url: Option<String>,
    /// The initial liquidity information.
    #[serde(rename = "init_liq")]
    pub init_liq: Option<InitLiq>,
    /// The name of the object.
    pub name: Option<String>,
    /// The timestamp when the object was opened.
    #[serde(rename = "open_timestamp")]
    pub open_timestamp: Option<i64>,
    /// The pooled SOL value.
    #[serde(rename = "pooled_sol")]
    pub pooled_sol: Option<f64>,
    /// The price in USD.
    #[serde(rename = "price_usd")]
    pub price_usd: Option<f64>,
    /// Indicates if the object is pump migrated.
    #[serde(rename = "pump_migrated")]
    pub pump_migrated: Option<bool>,
    /// The progress of the pump.
    #[serde(rename = "pump_progress")]
    pub pump_progress: Option<i64>,
    /// The count of sells.
    #[serde(rename = "sells_count")]
    pub sells_count: Option<i64>,
    /// The count of snipers.
    #[serde(rename = "snipers_count")]
    pub snipers_count: Option<i64>,
    /// The social media information.
    pub socials: Option<Socials>,
    /// The symbol of the object.
    pub symbol: Option<String>,
    /// The token address.
    pub token_address: Option<String>,
    /// The volume of the object.
    pub volume: Option<f64>,
    /// The action associated with the object.
    pub action: Option<String>,
}

/// Represents the audit information of a data object.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audit {
    /// Indicates if the freeze authority is enabled.
    #[serde(rename = "freeze_authority")]
    pub freeze_authority: Option<bool>,
    /// The percentage of LP burned.
    #[serde(rename = "lp_burned_perc")]
    pub lp_burned_perc: i64,
    /// Indicates if the mint authority is enabled.
    #[serde(rename = "mint_authority")]
    pub mint_authority: Option<bool>,
    /// The percentage of top holders.
    #[serde(rename = "top_holders_perc")]
    pub top_holders_perc: f64,
}

/// Represents the current liquidity information.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurLiq {
    /// The quote value.
    pub quote: f64,
    /// The USD value.
    pub usd: Option<f64>,
}

/// Represents the initial liquidity information.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitLiq {
    /// The amount of LP.
    #[serde(rename = "lp_amount")]
    pub lp_amount: Option<i64>,
    /// The timestamp when the object was opened.
    #[serde(rename = "open_timestamp")]
    pub open_timestamp: Option<i64>,
    /// The percentage value.
    pub percentage: Option<f64>,
    /// The quote value.
    pub quote: Option<f64>,
    /// The timestamp value.
    pub timestamp: Option<i64>,
    /// The token value.
    pub token: Option<f64>,
    /// The USD value.
    pub usd: Option<Value>,
    /// Indicates if the object is pending.
    pub pending: Option<bool>,
}

/// Represents the social media information.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Socials {
    /// The Medium URL.
    pub medium: Option<Value>,
    /// The Reddit URL.
    pub reddit: Option<Value>,
    /// The Telegram URL.
    pub telegram: Option<String>,
    /// The Twitter URL.
    pub twitter: Option<String>,
    /// The website URL.
    pub website: Option<String>,
}
