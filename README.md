# IBKR NASDAQ Option Scanner

A Rust-based scanner for finding profitable options on NASDAQ stocks using Interactive Brokers Web API.

## Features

- Scans option chains for NASDAQ stocks
- Calculates potential profit percentage for each option
- Filters options based on minimum profit threshold (configurable)
- Logs all scanned options to CSV files
- Real-time connection to IBKR Gateway
- Configurable scan intervals
- Detailed logging and reporting

## Quick Start

1. Run the setup script:
   ```bash
   chmod +x setup.sh
   ./setup.sh
