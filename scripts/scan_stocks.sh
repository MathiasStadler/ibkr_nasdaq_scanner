#!/bin/bash
# scan_stocks.sh - Scan specific stocks for options

set -e

echo "IBKR Option Scanner - Custom Stock Scan"

# Check for stock symbols argument
if [ $# -eq 0 ]; then
    echo "Usage: $0 <stock_symbol1> [stock_symbol2 ...]"
    echo "Example: $0 AAPL MSFT GOOGL"
    exit 1
fi

# Join stock symbols with commas
STOCKS=$(IFS=,; echo "$*")

echo "Scanning stocks: $STOCKS"

# Create temporary environment file
TEMP_ENV=$(mktemp)
cat ../.env > "$TEMP_ENV"

# Update the stock list
if grep -q "NASDAQ_STOCKS=" "$TEMP_ENV"; then
    sed -i "s/^NASDAQ_STOCKS=.*/NASDAQ_STOCKS=$STOCKS/" "$TEMP_ENV"
else
    echo "NASDAQ_STOCKS=$STOCKS" >> "$TEMP_ENV"
fi

# Set the temporary environment file
export ENV_FILE="$TEMP_ENV"

echo ""
echo "Starting scanner with custom stock list..."
echo "Press Ctrl+C to stop"

# Run the scanner
cd ..
cargo run

# Clean up
rm -f "$TEMP_ENV"
