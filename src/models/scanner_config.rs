use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerConfig {
    pub ibkr_gateway_host: String,
    pub ibkr_gateway_port: u16,
    pub ibkr_gateway_ssl: bool,
    pub min_profit_percent: f64,
    pub scan_interval_seconds: u64,
    pub max_retry_attempts: u32,
    pub request_timeout_seconds: u64,
    pub nasdaq_stocks: Vec<String>,
    pub ibkr_account_id: String,
    pub log_file_path: String,
    pub options_log_dir: String,
}

impl ScannerConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            ibkr_gateway_host: env::var("IBKR_GATEWAY_HOST")
                .unwrap_or_else(|_| "localhost".to_string()),
            ibkr_gateway_port: env::var("IBKR_GATEWAY_PORT")
                .unwrap_or_else(|_| "5000".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidPort)?,
            ibkr_gateway_ssl: env::var("IBKR_GATEWAY_SSL")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            min_profit_percent: env::var("MIN_PROFIT_PERCENT")
                .unwrap_or_else(|_| "2.0".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidFloat)?,
            scan_interval_seconds: env::var("SCAN_INTERVAL_SECONDS")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidUint)?,
            max_retry_attempts: env::var("MAX_RETRY_ATTEMPTS")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidUint)?,
            request_timeout_seconds: env::var("REQUEST_TIMEOUT_SECONDS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidUint)?,
            nasdaq_stocks: env::var("NASDAQ_STOCKS")
                .unwrap_or_else(|_| "AAPL,MSFT,GOOGL".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            ibkr_account_id: env::var("IBKR_ACCOUNT_ID")
                .unwrap_or_else(|_| "".to_string()),
            log_file_path: env::var("LOG_FILE_PATH")
                .unwrap_or_else(|_| "./logs/scanner.log".to_string()),
            options_log_dir: env::var("OPTIONS_LOG_DIR")
                .unwrap_or_else(|_| "./logs/options/".to_string()),
        })
    }
    
    pub fn base_url(&self) -> String {
        let protocol = if self.ibkr_gateway_ssl { "https" } else { "http" };
        format!("{}://{}:{}", protocol, self.ibkr_gateway_host, self.ibkr_gateway_port)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid port number")]
    InvalidPort,
    #[error("Invalid floating point number")]
    InvalidFloat,
    #[error("Invalid unsigned integer")]
    InvalidUint,
    #[error("Missing required environment variable: {0}")]
    MissingEnv(String),
}
