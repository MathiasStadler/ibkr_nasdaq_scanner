use crate::api::IbkrClient;
use crate::models::{OptionChain, ScannerConfig, StockList};
use crate::scanner::profit_calculator::calculate_profit_percent;
use crate::scanner::logger::{OptionLogger, LoggerError};
use log::{error, info, warn};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScannerError {
    #[error("IBKR API error: {0}")]
    IbkrError(#[from] crate::api::IbkrError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Logging error: {0}")]
    LoggerError(#[from] LoggerError),
}

pub struct NasdaqScanner {
    client: Arc<IbkrClient>,
    config: ScannerConfig,
    logger: OptionLogger,
}

impl NasdaqScanner {
    pub fn new(client: IbkrClient, config: ScannerConfig) -> Result<Self, ScannerError> {
        let logger = OptionLogger::new(&config.options_log_dir)?;
        
        Ok(Self {
            client: Arc::new(client),
            config,
            logger,
        })
    }
    
    pub async fn run_scanner(&self) -> Result<(), ScannerError> {
        info!("Starting NASDAQ option scanner");
        
        // Test connection first
        if !self.client.test_connection().await? {
            return Err(ScannerError::ConfigError(
                "Failed to connect to IBKR Gateway".to_string(),
            ));
        }
        
        loop {
            info!("Starting scan cycle");
            
            let stocks = StockList::from_symbols(&self.config.nasdaq_stocks);
            let profitable_options = self.scan_stocks(&stocks).await?;
            
            // Log profitable options
            self.logger.log_options(&profitable_options)?;
            
            // Display results
            self.display_results(&profitable_options);
            
            info!(
                "Scan complete. Found {} profitable options. Next scan in {} seconds",
                profitable_options.len(),
                self.config.scan_interval_seconds
            );
            
            sleep(Duration::from_secs(self.config.scan_interval_seconds)).await;
        }
    }
    
    async fn scan_stocks(
        &self,
        stocks: &StockList,
    ) -> Result<Vec<OptionChain>, ScannerError> {
        let mut all_profitable_options = Vec::new();
        
        for stock in &stocks.stocks {
            info!("Scanning {}", stock.symbol);
            
            match self.scan_single_stock(stock).await {
                Ok(mut options) => {
                    all_profitable_options.append(&mut options);
                }
                Err(e) => {
                    error!("Failed to scan {}: {}", stock.symbol, e);
                }
            }
        }
        
        info!("Scan complete. Found {} profitable options", all_profitable_options.len());
        
        Ok(all_profitable_options)
    }
    
    async fn scan_single_stock(
        &self,
        stock: &crate::models::Stock,
    ) -> Result<Vec<OptionChain>, ScannerError> {
        let mut profitable_options = Vec::new();
        
        // Get stock price
        let stock_price = match self.client.get_stock_price(&stock.symbol).await {
            Ok(price) => price,
            Err(e) => {
                warn!("Failed to get price for {}: {}", stock.symbol, e);
                return Ok(profitable_options);
            }
        };
        
        // Get option chain (using a reasonable number of strikes)
        let option_chains = match self.client.get_option_chain(&stock.symbol, 20).await {
            Ok(chains) => chains,
            Err(e) => {
                warn!("Failed to get option chain for {}: {}", stock.symbol, e);
                return Ok(profitable_options);
            }
        };
        
        // Calculate profit for each option
        for mut chain in option_chains {
            chain.stock_price = stock_price;
            chain.profit_percent = calculate_profit_percent(
                chain.strike,
                chain.option_price,
                chain.stock_price,
                &chain.option_type,
            );
            
            if chain.profit_percent >= self.config.min_profit_percent {
                profitable_options.push(chain);
            }
        }
        
        info!(
            "Scanned {}: found {} profitable options",
            stock.symbol,
            profitable_options.len()
        );
        
        Ok(profitable_options)
    }
    
    fn display_results(&self, options: &[OptionChain]) {
        use prettytable::{Table, row};
        
        if options.is_empty() {
            println!("No profitable options found.");
            return;
        }
        
        let mut table = Table::new();
        table.add_row(row![
            "Symbol",
            "Type",
            "Expiry",
            "Strike",
            "Option Price",
            "Stock Price",
            "Profit %",
            "Volume",
            "IV %"
        ]);
        
        for option in options {
            table.add_row(row![
                option.symbol,
                format!("{} {}", option.option_type.emoji(), option.option_type),
                option.expiration.format("%Y-%m-%d"),
                format!("${:.2}", option.strike),
                format!("${:.2}", option.option_price),
                format!("${:.2}", option.stock_price),
                format!("{:.2}%", option.profit_percent),
                option.volume,
                format!("{:.1}%", option.implied_volatility * 100.0)
            ]);
        }
        
        println!("\nProfitable Options Found:");
        table.printstd();
    }
}

// Public run function that creates and runs the scanner
pub async fn run() -> Result<(), ScannerError> {
    let config = ScannerConfig::from_env()
        .map_err(|e| ScannerError::ConfigError(e.to_string()))?;
    
    let client = IbkrClient::new(
        config.base_url(),
        config.ibkr_account_id.clone(),
        config.request_timeout_seconds,
    );
    
    let scanner = NasdaqScanner::new(client, config)?;
    scanner.run_scanner().await
}
