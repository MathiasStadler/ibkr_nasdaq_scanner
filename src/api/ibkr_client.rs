use crate::models::{OptionChain, OptionType};
use reqwest::{Client, Response};
use serde_json::Value;
use std::time::Duration;
use thiserror::Error;
use log::{error, info, warn};

#[derive(Debug, Clone)]
pub struct IbkrClient {
    client: Client,
    base_url: String,
    account_id: String,
}

#[derive(Debug, Error)]
pub enum IbkrError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("API error: {0}")]
    ApiError(String),
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Invalid data format: {0}")]
    DataError(String),
}

impl IbkrClient {
    pub fn new(base_url: String, account_id: String, timeout_seconds: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");
        
        Self {
            client,
            base_url,
            account_id,
        }
    }
    
    pub async fn get_option_chain(
        &self,
        symbol: &str,
        strike_count: usize,
    ) -> Result<Vec<OptionChain>, IbkrError> {
        info!("Fetching option chain for {}", symbol);
        
        // In a real implementation, this would call the actual IBKR API
        // For now, we'll simulate the response format
        let url = format!("{}/v1/api/iserver/secdef/info", self.base_url);
        
        let params = [
            ("conid", "265598"), // Example contract ID for AAPL
            ("sectype", "OPT"),
            ("month", "ALL"),
            ("exchange", "SMART"),
            ("strikeCount", &strike_count.to_string()),
        ];
        
        let response = self.client.get(&url)
            .query(&params)
            .send()
            .await
            .map_err(IbkrError::RequestError)?;
        
        self.handle_response(response).await
    }
    
    pub async fn get_stock_price(&self, symbol: &str) -> Result<f64, IbkrError> {
        info!("Fetching stock price for {}", symbol);
        
        // This would call the actual IBKR market data API
        let url = format!("{}/v1/api/iserver/marketdata/snapshot", self.base_url);
        
        let params = [
            ("conids", "265598"), // Example contract ID for AAPL
            ("fields", "31"), // Field 31 is last price
        ];
        
        let response = self.client.get(&url)
            .query(&params)
            .send()
            .await
            .map_err(IbkrError::RequestError)?;
        
        let json: Value = response.json().await?;
        
        if let Some(price) = json[0]["31"].as_f64() {
            Ok(price)
        } else {
            Err(IbkrError::DataError("Price not found in response".to_string()))
        }
    }
    
    pub async fn test_connection(&self) -> Result<bool, IbkrError> {
        info!("Testing connection to IBKR Gateway");
        
        let url = format!("{}/v1/api/tickle", self.base_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => {
                let status = response.status();
                if status.is_success() {
                    info!("Connection successful");
                    Ok(true)
                } else {
                    warn!("Connection failed with status: {}", status);
                    Ok(false)
                }
            }
            Err(e) => {
                error!("Connection error: {}", e);
                Err(IbkrError::RequestError(e))
            }
        }
    }
    
    async fn handle_response(&self, response: Response) -> Result<Vec<OptionChain>, IbkrError> {
        if !response.status().is_success() {
            return Err(IbkrError::ApiError(format!(
                "API request failed with status: {}",
                response.status()
            )));
        }
        
        let json: Value = response.json().await?;
        
        // Parse the response into OptionChain objects
        // This is a simplified parsing - actual IBKR response would be more complex
        self.parse_option_chain_response(json)
    }
    
    fn parse_option_chain_response(&self, json: Value) -> Result<Vec<OptionChain>, IbkrError> {
        // This is a mock implementation
        // In reality, you would parse the actual IBKR response format
        let mut chains = Vec::new();
        
        // For demonstration, create mock data
        // Real implementation would extract from JSON
        if let Some(calls) = json.get("calls") {
            if let Some(call_array) = calls.as_array() {
                for call in call_array {
                    if let Some(chain) = self.parse_option_data(call, OptionType::Call) {
                        chains.push(chain);
                    }
                }
            }
        }
        
        if let Some(puts) = json.get("puts") {
            if let Some(put_array) = puts.as_array() {
                for put in put_array {
                    if let Some(chain) = self.parse_option_data(put, OptionType::Put) {
                        chains.push(chain);
                    }
                }
            }
        }
        
        Ok(chains)
    }
    
    fn parse_option_data(&self, data: &Value, option_type: OptionType) -> Option<OptionChain> {
        // Parse individual option data
        // This is a simplified version
        Some(OptionChain {
            symbol: data.get("symbol")?.as_str()?.to_string(),
            option_type,
            expiration: chrono::Utc::now(), // Would parse actual date
            strike: data.get("strike")?.as_f64()?,
            option_price: data.get("bid")?.as_f64()?,
            stock_price: 0.0, // Will be populated separately
            profit_percent: 0.0, // Will be calculated
            volume: data.get("volume")?.as_i64()? as i32,
            implied_volatility: data.get("iv")?.as_f64()?,
            timestamp: chrono::Utc::now(),
            contract_id: data.get("conid")?.as_str()?.to_string(),
        })
    }
}
