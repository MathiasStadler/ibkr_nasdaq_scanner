use crate::models::OptionChain;
use chrono::{DateTime, Utc};
use csv::Writer;
use serde::Serialize;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoggerError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),
}

#[derive(Debug, Serialize)]
struct OptionLogEntry {
    timestamp: DateTime<Utc>,
    symbol: String,
    option_type: String,
    expiration: DateTime<Utc>,
    strike: f64,
    option_price: f64,
    stock_price: f64,
    profit_percent: f64,
    volume: i32,
    implied_volatility: f64,
    contract_id: String,
    type_emoji: String,
}

pub struct OptionLogger {
    log_dir: PathBuf,
}

impl OptionLogger {
    pub fn new(log_dir: &str) -> Result<Self, LoggerError> {
        let path = PathBuf::from(log_dir);
        
        // Create directory if it doesn't exist
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        
        Ok(Self { log_dir: path })
    }
    
    pub fn log_options(&self, options: &[OptionChain]) -> Result<(), LoggerError> {
        if options.is_empty() {
            return Ok(());
        }
        
        let timestamp = Utc::now();
        let date_str = timestamp.format("%Y%m%d").to_string();
        let filename = format!("options_{}.csv", date_str);
        let filepath = self.log_dir.join(filename);
        
        let file_exists = filepath.exists();
        
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&filepath)?;
        
        let mut wtr = Writer::from_writer(file);
        
        // Write header if file is new
        if !file_exists {
            wtr.write_record(&[
                "timestamp",
                "symbol",
                "option_type",
                "type_emoji",
                "expiration",
                "strike",
                "option_price",
                "stock_price",
                "profit_percent",
                "volume",
                "implied_volatility",
                "contract_id",
            ])?;
        }
        
        for option in options {
            let entry = OptionLogEntry {
                timestamp: option.timestamp,
                symbol: option.symbol.clone(),
                option_type: option.option_type.to_string(),
                expiration: option.expiration,
                strike: option.strike,
                option_price: option.option_price,
                stock_price: option.stock_price,
                profit_percent: option.profit_percent,
                volume: option.volume,
                implied_volatility: option.implied_volatility,
                contract_id: option.contract_id.clone(),
                type_emoji: option.option_type.emoji().to_string(),
            };
            
            wtr.serialize(entry)?;
        }
        
        wtr.flush()?;
        
        // Also create a summary log for today
        self.create_summary_log(options, timestamp)?;
        
        Ok(())
    }
    
    fn create_summary_log(
        &self,
        options: &[OptionChain],
        timestamp: DateTime<Utc>,
    ) -> Result<(), LoggerError> {
        let date_str = timestamp.format("%Y%m%d").to_string();
        let summary_filename = format!("summary_{}.txt", date_str);
        let summary_path = self.log_dir.join(summary_filename);
        
        let mut file = File::create(summary_path)?;
        
        writeln!(file, "Option Scan Summary - {}", timestamp)?;
        writeln!(file, "======================================")?;
        writeln!(file, "Total profitable options found: {}", options.len())?;
        writeln!(file)?;
        
        // Group by symbol
        use std::collections::HashMap;
        let mut by_symbol: HashMap<String, Vec<&OptionChain>> = HashMap::new();
        
        for option in options {
            by_symbol.entry(option.symbol.clone())
                .or_default()
                .push(option);
        }
        
        for (symbol, symbol_options) in by_symbol {
            writeln!(file, "{}: {} options", symbol, symbol_options.len())?;
            
            for option in symbol_options {
                writeln!(
                    file,
                    "  {} {}: Strike ${:.2}, Option ${:.2}, Profit {:.2}%",
                    option.option_type.emoji(),
                    option.option_type,
                    option.strike,
                    option.option_price,
                    option.profit_percent
                )?;
            }
            writeln!(file)?;
        }
        
        Ok(())
    }
}
