use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "type")]
    pub type_field: String,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub attributes: Attributes,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub address: Option<String>,
    pub audit: Option<Audit>,
    #[serde(rename = "buys_count")]
    pub buys_count: Option<i64>,
    #[serde(rename = "created_timestamp")]
    pub created_timestamp: Option<i64>,
    #[serde(rename = "cur_liq")]
    pub cur_liq: Option<CurLiq>,
    #[serde(rename = "dev_holding_perc")]
    pub dev_holding_perc: Option<f64>,
    #[serde(rename = "dex_i")]
    pub dex_i: Option<i64>,
    pub fdv: Option<f64>,
    pub from_meme_dex: Option<i64>,
    pub from_moonshot: Option<bool>,
    pub from_pump: Option<bool>,
    #[serde(rename = "from_meme_dex")]
    pub from_meme_dex2: Option<i64>,
    #[serde(rename = "holders_count")]
    pub holders_count: Option<i64>,
    pub ignored: Option<bool>,
    pub img_url: Option<String>,
    #[serde(rename = "init_liq")]
    pub init_liq: Option<InitLiq>,
    pub name: Option<String>,
    #[serde(rename = "open_timestamp")]
    pub open_timestamp: Option<i64>,
    #[serde(rename = "pooled_sol")]
    pub pooled_sol: Option<f64>,
    #[serde(rename = "price_usd")]
    pub price_usd: Option<f64>,
    #[serde(rename = "pump_migrated")]
    pub pump_migrated: Option<bool>,
    #[serde(rename = "pump_progress")]
    pub pump_progress: Option<i64>,
    #[serde(rename = "sells_count")]
    pub sells_count: Option<i64>,
    #[serde(rename = "snipers_count")]
    pub snipers_count: Option<i64>,
    pub socials: Option<Socials>,
    pub symbol: Option<String>,
    pub token_address: Option<String>,
    pub volume: Option<f64>,
    pub action: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audit {
    #[serde(rename = "freeze_authority")]
    pub freeze_authority: Option<bool>,
    #[serde(rename = "lp_burned_perc")]
    pub lp_burned_perc: i64,
    #[serde(rename = "mint_authority")]
    pub mint_authority: Option<bool>,
    #[serde(rename = "top_holders_perc")]
    pub top_holders_perc: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurLiq {
    pub quote: f64,
    pub usd: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitLiq {
    #[serde(rename = "lp_amount")]
    pub lp_amount: Option<i64>,
    #[serde(rename = "open_timestamp")]
    pub open_timestamp: Option<i64>,
    pub percentage: Option<f64>,
    pub quote: Option<f64>,
    pub timestamp: Option<i64>,
    pub token: Option<f64>,
    pub usd: Option<Value>,
    pub pending: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Socials {
    pub medium: Option<Value>,
    pub reddit: Option<Value>,
    pub telegram: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
}
