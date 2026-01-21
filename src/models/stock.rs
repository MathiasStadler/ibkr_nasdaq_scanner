use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stock {
    pub symbol: String,
    pub name: String,
    pub current_price: f64,
    pub market_cap: Option<f64>,
    pub sector: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockList {
    pub stocks: Vec<Stock>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl StockList {
    pub fn from_symbols(symbols: &[String]) -> Self {
        let stocks = symbols
            .iter()
            .map(|symbol| Stock {
                symbol: symbol.clone(),
                name: "Unknown".to_string(),
                current_price: 0.0,
                market_cap: None,
                sector: None,
            })
            .collect();
        
        Self {
            stocks,
            last_updated: chrono::Utc::now(),
        }
    }
}
