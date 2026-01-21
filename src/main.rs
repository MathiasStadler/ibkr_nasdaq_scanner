mod api;
mod models;
mod scanner;
mod utils;

use dotenv::dotenv;
use env_logger;
use log::{error, info};
use std::process;

#[tokio::main]
async fn main() {
    // Initialize environment variables
    dotenv().ok();
    
    // Initialize logger
    env_logger::init();
    
    info!("Starting IBKR NASDAQ Option Scanner");
    
    // Run the scanner - fixed path
    if let Err(e) = scanner::nasdaq_scanner::run().await {
        error!("Scanner failed: {}", e);
        process::exit(1);
    }
}
