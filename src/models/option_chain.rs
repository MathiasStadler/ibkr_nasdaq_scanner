use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionChain {
    pub symbol: String,
    pub option_type: OptionType,
    pub expiration: DateTime<Utc>,
    pub strike: f64,
    pub option_price: f64,
    pub stock_price: f64,
    pub profit_percent: f64,
    pub volume: i32,
    pub implied_volatility: f64,
    pub timestamp: DateTime<Utc>,
    pub contract_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OptionType {
    Call,
    Put,
}

impl fmt::Display for OptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionType::Call => write!(f, "CALL"),
            OptionType::Put => write!(f, "PUT"),
        }
    }
}

impl OptionType {
    pub fn emoji(&self) -> &'static str {
        match self {
            OptionType::Call => "📈",
            OptionType::Put => "📉",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionChainFilters {
    pub min_profit_percent: f64,
    pub min_volume: i32,
    pub max_days_to_expiry: i32,
}

impl Default for OptionChainFilters {
    fn default() -> Self {
        Self {
            min_profit_percent: 2.0,
            min_volume: 100,
            max_days_to_expiry: 45,
        }
    }
}
