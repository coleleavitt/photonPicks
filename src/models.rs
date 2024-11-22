use serde::{Deserialize, Serialize};

/// Represents a response containing token data.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    /// The type of the message.
    #[serde(rename = "type")]
    pub message_type: String,
    /// A list of token data.
    pub data: Vec<TokenData>,
}

/// Represents the data of a token.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    /// The attributes of the token.
    pub attributes: TokenAttributes,
    /// The ID of the token.
    pub id: String,
    /// The type of the data.
    #[serde(rename = "type")]
    pub data_type: String,
}

/// Represents the attributes of a token.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAttributes {
    /// The address of the token.
    pub address: String,
    /// The audit information of the token.
    pub audit: TokenAudit,
    /// The number of buys.
    pub buys_count: i64,
    /// The timestamp when the token was created.
    pub created_timestamp: i64,
    /// The current liquidity of the token.
    pub cur_liq: Liquidity,
    /// The percentage of developer holdings.
    #[serde(default, with = "string_or_float_option")]
    pub dev_holding_perc: Option<f64>,
    /// The DEX index.
    pub dex_i: i64,
    /// The fully diluted valuation.
    #[serde(default, with = "string_or_float")]
    pub fdv: f64,
    /// The number of holders.
    pub holders_count: i64,
    /// The name of the token.
    pub name: String,
    /// The price of the token in USD.
    #[serde(default, with = "string_or_float_option")]
    pub price_usd: Option<f64>,
    /// The symbol of the token.
    pub symbol: String,
    /// The trading volume of the token.
    #[serde(with = "string_or_float")]
    pub volume: f64,
    /// The social media information of the token.
    #[serde(default)]
    pub socials: Option<Socials>,
    /// The number of sells.
    #[serde(default)]
    pub sells_count: Option<i64>,
    /// The amount of pooled SOL.
    #[serde(with = "string_or_float")]
    pub pooled_sol: f64,
}

/// Represents the audit information of a token.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAudit {
    /// Indicates if the freeze authority is enabled.
    pub freeze_authority: bool,
    /// The percentage of LP burned.
    #[serde(with = "string_or_float")]
    pub lp_burned_perc: f64,
    /// Indicates if the mint authority is enabled.
    pub mint_authority: bool,
    /// The percentage of top holders.
    #[serde(with = "string_or_float")]
    pub top_holders_perc: f64,
}

/// Represents the liquidity information of a token.
#[derive(Debug, Serialize, Deserialize)]
pub struct Liquidity {
    /// The quote value of the liquidity.
    #[serde(with = "string_or_float")]
    pub quote: f64,
    /// The USD value of the liquidity.
    pub usd: String,
}

/// Represents the social media information of a token.
#[derive(Debug, Serialize, Deserialize)]
pub struct Socials {
    /// The Medium account of the token.
    #[serde(default)]
    pub medium: Option<String>,
    /// The Reddit account of the token.
    #[serde(default)]
    pub reddit: Option<String>,
    /// The Telegram account of the token.
    #[serde(default)]
    pub telegram: Option<String>,
    /// The Twitter account of the token.
    #[serde(default)]
    pub twitter: Option<String>,
    /// The website of the token.
    #[serde(default)]
    pub website: Option<String>,
}

/// Custom serializer/deserializer for handling both string and float values.
mod string_or_float {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    /// Serializes a float value as a string.
    pub fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    /// Deserializes a value that can be either a string or a float.
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

/// Custom serializer/deserializer for handling both string and float values for Option<f64>.
mod string_or_float_option {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    /// Serializes an Option<f64> value as a string.
    pub fn serialize<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => serializer.serialize_str(&v.to_string()),
            None => serializer.serialize_none(),
        }
    }

    /// Deserializes a value that can be either a string, a float, or null.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
            Null,
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => Ok(Some(f64::from_str(&s).map_err(serde::de::Error::custom)?)),
            StringOrFloat::Float(f) => Ok(Some(f)),
            StringOrFloat::Null => Ok(None),
        }
    }
}