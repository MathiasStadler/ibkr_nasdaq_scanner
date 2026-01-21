#!/bin/bash
# get_running_trades.sh - Get current trades from IBKR

set -e

echo "Fetching running trades from IBKR..."

# Load environment variables
if [ -f ../.env ]; then
    export $(grep -v '^#' ../.env | xargs)
else
    echo "Error: .env file not found"
    exit 1
fi

# Construct the gateway URL
if [ "$IBKR_GATEWAY_SSL" = "true" ]; then
    PROTOCOL="https"
else
    PROTOCOL="http"
fi

GATEWAY_URL="${PROTOCOL}://${IBKR_GATEWAY_HOST}:${IBKR_GATEWAY_PORT}"

echo "Using Gateway URL: $GATEWAY_URL"
echo "Account ID: $IBKR_ACCOUNT_ID"

# Get portfolio accounts
echo ""
echo "=== Portfolio Accounts ==="
curl -s "$GATEWAY_URL/v1/api/portfolio/accounts" | jq '.' || echo "Failed to get accounts"

# Get positions (requires authenticated session)
echo ""
echo "=== Current Positions ==="
curl -s "$GATEWAY_URL/v1/api/portfolio/positions" | jq '.' || echo "Failed to get positions"

# Get orders
echo ""
echo "=== Open Orders ==="
curl -s "$GATEWAY_URL/v1/api/iserver/account/orders" | jq '.' || echo "Failed to get orders"

# Get trades
echo ""
echo "=== Recent Trades ==="
curl -s "$GATEWAY_URL/v1/api/iserver/account/trades" | jq '.' || echo "Failed to get trades"
