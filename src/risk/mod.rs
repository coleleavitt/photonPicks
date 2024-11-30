use std::cmp::Ordering;
// risk.rs or risk/mod.rs
use crate::math::{collect_recent_trades, generate_wallet_holdings};
use crate::models::TokenData;
use eframe::egui;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RiskLevel {
    Low,
    Moderate,
    High,
    VeryHigh,
    Unknown,
}

// Implement Ord and PartialOrd for RiskLevel
impl PartialOrd for RiskLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RiskLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_value = match self {
            RiskLevel::Low => 0,
            RiskLevel::Moderate => 1,
            RiskLevel::High => 2,
            RiskLevel::VeryHigh => 3,
            RiskLevel::Unknown => 4,
        };
        let other_value = match other {
            RiskLevel::Low => 0,
            RiskLevel::Moderate => 1,
            RiskLevel::High => 2,
            RiskLevel::VeryHigh => 3,
            RiskLevel::Unknown => 4,
        };
        self_value.cmp(&other_value)
    }
}

#[derive(Clone)]
pub struct RiskCalculator {
    thresholds: HashMap<RiskLevel, f64>,
}

impl Default for RiskCalculator {
    fn default() -> Self {
        let mut thresholds = HashMap::new();
        thresholds.insert(RiskLevel::Low, 0.25);
        thresholds.insert(RiskLevel::Moderate, 0.50);
        thresholds.insert(RiskLevel::High, 0.75);
        thresholds.insert(RiskLevel::VeryHigh, 1.0);
        Self { thresholds }
    }
}

impl RiskCalculator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn calculate_risk(&self, token: &TokenData) -> RiskLevel {
        let holdings = generate_wallet_holdings(token);
        let trades = collect_recent_trades(token);
        let concentration = token.calculate_adjusted_concentration(&holdings, &trades);
        self.get_risk_level(concentration)
    }

    pub fn get_risk_level(&self, concentration: f64) -> RiskLevel {
        if concentration <= self.thresholds[&RiskLevel::Low] {
            RiskLevel::Low
        } else if concentration <= self.thresholds[&RiskLevel::Moderate] {
            RiskLevel::Moderate
        } else if concentration <= self.thresholds[&RiskLevel::High] {
            RiskLevel::High
        } else if concentration <= self.thresholds[&RiskLevel::VeryHigh] {
            RiskLevel::VeryHigh
        } else {
            RiskLevel::Unknown
        }
    }

    pub fn get_risk_color(&self, risk: RiskLevel) -> egui::Color32 {
        match risk {
            RiskLevel::Low => egui::Color32::from_rgb(20, 110, 20),
            RiskLevel::Moderate => egui::Color32::from_rgb(110, 110, 20),
            RiskLevel::High => egui::Color32::from_rgb(110, 60, 20),
            RiskLevel::VeryHigh => egui::Color32::from_rgb(110, 20, 20),
            RiskLevel::Unknown => egui::Color32::from_gray(128),
        }
    }

    pub fn get_risk_text(&self, risk: RiskLevel) -> &'static str {
        match risk {
            RiskLevel::Low => "Low Risk",
            RiskLevel::Moderate => "Moderate Risk",
            RiskLevel::High => "High Risk",
            RiskLevel::VeryHigh => "Very High Risk",
            RiskLevel::Unknown => "Unknown Risk",
        }
    }
}
