use crate::models::TokenData;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug)]
pub struct TradePattern {
    pub timestamp: SystemTime,
    pub price: f64,
    pub amount: f64,
    pub wallet: String,
    pub trade_type: TradeType,
}

#[derive(Debug)]
pub enum TradeType {
    Buy,
    Sell,
}

impl TokenData {
    pub fn calculate_wallet_concentration(&self, wallet_holdings: &HashMap<String, f64>) -> f64 {
        let total_supply: f64 = wallet_holdings.values().sum();

        if total_supply <= 0.0 || wallet_holdings.is_empty() {
            return 0.0;
        }

        let hhi: f64 = wallet_holdings
            .values()
            .map(|&amount| {
                let market_share = amount / total_supply;
                market_share * market_share
            })
            .sum();

        if let Some(n) = self.attributes.holders_count {
            let n = n as f64;
            return (hhi - (1.0 / n)) / (1.0 - (1.0 / n));
        }

        hhi
    }

    pub fn detect_bot_patterns(&self, trades: &[TradePattern]) -> f64 {
        let mut clustered_trades: HashMap<u64, Vec<&TradePattern>> = HashMap::new();
        let mut bot_likelihood: f64 = 0.0;

        // Add initial likelihood based on token attributes
        if let Some(sniper_count) = self.attributes.snipers_count {
            if sniper_count > 0 {
                bot_likelihood += 0.2;
            }
        }

        // Group trades by timestamp in seconds
        for trade in trades {
            if let Ok(timestamp) = trade.timestamp.duration_since(SystemTime::UNIX_EPOCH) {
                let key = timestamp.as_secs();
                clustered_trades
                    .entry(key)
                    .or_default()
                    .push(trade);
            }
        }

        for (_timestamp, cluster) in clustered_trades {
            if cluster.len() >= 3 {
                // Lower threshold since bots often trade in smaller bursts
                let price_variance = calculate_variance(cluster.iter().map(|t| t.price));
                let amount_variance = calculate_variance(cluster.iter().map(|t| t.amount));

                // Check for consistent trade sizes and prices (tightened thresholds)
                if price_variance < 0.00001 && amount_variance < 0.001 {
                    bot_likelihood += 0.3;
                }

                // Check trade type patterns
                let all_same_type = cluster.windows(2).all(|w| {
                    std::mem::discriminant(&w[0].trade_type)
                        == std::mem::discriminant(&w[1].trade_type)
                });
                if all_same_type {
                    bot_likelihood += 0.2;
                }

                // Check for wallet pattern repetition with stricter criteria
                let unique_wallets = cluster
                    .iter()
                    .map(|t| &t.wallet)
                    .collect::<std::collections::HashSet<_>>();
                if unique_wallets.len() <= 3 && cluster.len() > 5 {
                    bot_likelihood += 0.3;
                }
            }
        }

        f64::min(bot_likelihood, 1.0)
    }

    pub fn calculate_adjusted_concentration(
        &self,
        wallet_holdings: &HashMap<String, f64>,
        trades: &[TradePattern],
    ) -> f64 {
        let base_hhi = self.calculate_wallet_concentration(wallet_holdings);
        let bot_likelihood = self.detect_bot_patterns(trades);

        // Exponential adjustment for bot activity
        base_hhi * (1.0 + bot_likelihood.powf(1.5))
    }

    pub fn get_concentration_risk(&self, hhi: f64) -> &str {
        match hhi {
            h if h < 0.15 => "Low Risk",
            h if h < 0.25 => "Moderate Risk",
            h if h < 0.40 => "High Risk",
            _ => "Very High Risk",
        }
    }
}

fn calculate_variance<I>(values: I) -> f64
where
    I: Iterator<Item = f64>,
{
    let mut count = 0;
    let mut sum = 0.0;
    let mut sum_sq = 0.0;

    for value in values {
        count += 1;
        sum += value;
        sum_sq += value * value;
    }

    if count > 0 {
        let mean = sum / count as f64;
        (sum_sq / count as f64) - (mean * mean)
    } else {
        0.0
    }
}

pub(crate) fn generate_wallet_holdings(token: &TokenData) -> HashMap<String, f64> {
    let mut holdings = HashMap::new();

    if let Some(ref audit) = token.attributes.audit {
        let top_holders = audit.top_holders_perc;

        if let Some(holders_count) = token.attributes.holders_count {
            let top_n = (f64::from(holders_count) * 0.1).ceil() as u32;
            for i in 0..top_n {
                let share = top_holders * (1.0 - 0.1 * f64::from(i));
                holdings.insert(format!("holder_{}", i), share);
            }

            let remaining = 100.0 - holdings.values().sum::<f64>();
            let remaining_holders = holders_count.saturating_sub(top_n);
            if remaining_holders > 0 {
                holdings.insert(
                    "remaining_holders".to_string(),
                    remaining / f64::from(remaining_holders),
                );
            }
        }
    }

    holdings
}

pub fn collect_recent_trades(token: &TokenData) -> Vec<TradePattern> {
    let mut trades = Vec::new();
    let timestamp = SystemTime::now();

    // Add buy trades
    if let Some(buys_count) = token.attributes.buys_count {
        if let Some(price) = token.attributes.price_usd {
            let avg_amount = token.attributes.volume.unwrap_or(0.0) / (buys_count as f64);

            for i in 0..buys_count.min(20) {
                trades.push(TradePattern {
                    timestamp,
                    price,
                    amount: avg_amount,
                    wallet: format!("buyer_{}", i),
                    trade_type: TradeType::Buy,
                });
            }
        }
    }

    // Add sell trades
    if let Some(sells_count) = token.attributes.sells_count {
        if let Some(price) = token.attributes.price_usd {
            let avg_amount = token.attributes.volume.unwrap_or(0.0) / (sells_count as f64);

            for i in 0..sells_count.min(20) {
                trades.push(TradePattern {
                    timestamp,
                    price,
                    amount: avg_amount,
                    wallet: format!("seller_{}", i),
                    trade_type: TradeType::Sell,
                });
            }
        }
    }

    trades
}
