#!/bin/bash
# test_connection.sh - Test IBKR Gateway Connection

set -e

echo "Testing IBKR Gateway Connection..."

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

echo "Testing connection to: $GATEWAY_URL"

# Test basic connectivity
if curl -s --connect-timeout 10 "$GATEWAY_URL/v1/api/tickle" > /dev/null; then
    echo "✅ Connection successful"
    
    # Try to get account information
    echo "Testing API endpoints..."
    
    # Test market data (requires authentication in real scenario)
    if curl -s --connect-timeout 10 "$GATEWAY_URL/v1/api/iserver/auth/status" > /dev/null; then
        echo "✅ Authentication endpoint accessible"
    else
        echo "⚠️  Authentication endpoint may require login"
    fi
    
    exit 0
else
    echo "❌ Connection failed"
    echo ""
    echo "Troubleshooting steps:"
    echo "1. Ensure IBKR Gateway is running"
    echo "2. Check if gateway is listening on $IBKR_GATEWAY_HOST:$IBKR_GATEWAY_PORT"
    echo "3. Verify firewall settings"
    echo "4. Check gateway logs for errors"
    exit 1
fi
