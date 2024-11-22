use crate::models::{TokenAttributes, TokenData};

// src/token_analysis.rs
pub struct TokenFilter {
    min_market_cap: f64,
    max_market_cap: f64,
    max_top_holders_perc: f64,
    min_buy_sell_ratio: f64,
    min_volume: f64,
    min_pooled_sol: f64,
}

impl Default for TokenFilter {
    fn default() -> Self {
        Self {
            min_market_cap: 40000.0,
            max_market_cap: 500000.0,
            max_top_holders_perc: 25.0,
            min_buy_sell_ratio: 1.2,
            min_volume: 5000.0,
            min_pooled_sol: 20.0,
        }
    }
}

impl TokenFilter {
    pub fn has_positive_momentum(&self, attributes: &TokenAttributes) -> bool {
        let buy_to_sell_ratio =
            attributes.buys_count as f64 / (attributes.sells_count.unwrap_or(1) as f64);
        let volume_to_mcap_ratio = attributes.volume / attributes.fdv;

        buy_to_sell_ratio >= self.min_buy_sell_ratio
            && attributes.volume >= self.min_volume
            && attributes.pooled_sol >= self.min_pooled_sol
            && volume_to_mcap_ratio > 0.1
    }

    pub fn meets_criteria(&self, token: &TokenData) -> bool {
        let attrs = &token.attributes;

        // Basic checks
        if attrs.fdv < self.min_market_cap || attrs.fdv > self.max_market_cap {
            return false;
        }

        // Check top holders percentage
        if attrs.audit.top_holders_perc > self.max_top_holders_perc {
            return false;
        }

        // Check for Twitter presence
        if attrs
            .socials
            .as_ref()
            .and_then(|s| s.twitter.as_ref())
            .is_none()
        {
            return false;
        }

        self.has_positive_momentum(attrs)
    }
}

#[derive(Debug)]
pub struct TokenMetrics {
    pub name: String,
    pub symbol: String,
    pub price_usd: Option<f64>,
    pub market_cap: f64,
    pub holders: i64,
    pub top_holders_perc: f64,
    pub volume: f64,
    pub volume_mcap_ratio: f64,
    pub buy_sell_ratio: f64,
    pub age_hours: f64,
}

impl TokenMetrics {
    pub fn from_token(token: &TokenData) -> Self {
        let attrs = &token.attributes;
        let current_timestamp = chrono::Utc::now().timestamp();
        let age_hours = (current_timestamp - attrs.created_timestamp) as f64 / 3600.0;

        Self {
            name: attrs.name.clone(),
            symbol: attrs.symbol.clone(),
            price_usd: attrs.price_usd,
            market_cap: attrs.fdv,
            holders: attrs.holders_count,
            top_holders_perc: attrs.audit.top_holders_perc,
            volume: attrs.volume,
            volume_mcap_ratio: attrs.volume / attrs.fdv,
            buy_sell_ratio: attrs.buys_count as f64 / (attrs.sells_count.unwrap_or(1) as f64),
            age_hours,
        }
    }
}
