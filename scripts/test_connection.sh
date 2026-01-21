#!/bin/bash
# test_connection.sh - Test IBKR Gateway Connection - FIXED VERSION

set -e

echo "Testing IBKR Gateway Connection..."

# Load environment variables
if [ -f ../.env ]; then
    # Use a safer method to load env vars
    while IFS='=' read -r key value; do
        # Skip comments and empty lines
        [[ $key =~ ^#.*$ ]] || [[ -z $key ]] && continue
        # Remove quotes from value
        value=$(echo "$value" | sed "s/^['\"]//;s/['\"]$//")
        export "$key=$value"
    done < ../.env
else
    echo "Error: .env file not found in parent directory"
    exit 1
fi

# Set defaults if not set
IBKR_GATEWAY_HOST=${IBKR_GATEWAY_HOST:-localhost}
IBKR_GATEWAY_PORT=${IBKR_GATEWAY_PORT:-5000}
IBKR_GATEWAY_SSL=${IBKR_GATEWAY_SSL:-false}

echo "Configuration loaded:"
echo "  Host: $IBKR_GATEWAY_HOST"
echo "  Port: $IBKR_GATEWAY_PORT"
echo "  SSL: $IBKR_GATEWAY_SSL"

# Construct the gateway URL
if [[ "$IBKR_GATEWAY_SSL" == "true" ]]; then
    PROTOCOL="https"
    SSL_FLAG="--insecure"  # Allow self-signed certs
else
    PROTOCOL="http"
    SSL_FLAG=""
fi

GATEWAY_URL="${PROTOCOL}://${IBKR_GATEWAY_HOST}:${IBKR_GATEWAY_PORT}"
echo "Testing connection to: $GATEWAY_URL"

# Test basic connectivity with timeout
echo ""
echo "Step 1: Testing basic connectivity..."
if curl -s $SSL_FLAG --connect-timeout 10 "${GATEWAY_URL}/v1/api/tickle" > /dev/null; then
    echo "✅ Basic connectivity successful"
else
    echo "❌ Basic connectivity failed"
    echo ""
    echo "Troubleshooting:"
    echo "1. Check if gateway is running: netstat -tuln | grep :$IBKR_GATEWAY_PORT"
    echo "2. Try manual test: curl -v $GATEWAY_URL"
    echo "3. Check gateway logs for errors"
    echo "4. Verify firewall settings"
    exit 1
fi

# Test SSL if configured
if [[ "$IBKR_GATEWAY_SSL" == "true" ]]; then
    echo ""
    echo "Step 2: Testing SSL/TLS configuration..."
    if openssl s_client -connect "${IBKR_GATEWAY_HOST}:${IBKR_GATEWAY_PORT}" -servername "$IBKR_GATEWAY_HOST" < /dev/null 2>&1 | grep -q "Certificate chain"; then
        echo "✅ SSL/TLS certificate chain found"
    else
        echo "⚠️  SSL/TLS certificate check failed (may be self-signed)"
    fi
fi

# Test API endpoints
echo ""
echo "Step 3: Testing API endpoints..."

# Test tickle endpoint (always available)
echo -n "  Testing /v1/api/tickle... "
if curl -s $SSL_FLAG --connect-timeout 10 "${GATEWAY_URL}/v1/api/tickle" | grep -q "tickle"; then
    echo "✅ OK"
else
    echo "⚠️  Response unexpected (but connection works)"
fi

# Test auth status (may require login)
echo -n "  Testing /v1/api/iserver/auth/status... "
AUTH_RESPONSE=$(curl -s $SSL_FLAG --connect-timeout 10 "${GATEWAY_URL}/v1/api/iserver/auth/status" 2>/dev/null || true)
if [ -n "$AUTH_RESPONSE" ]; then
    if echo "$AUTH_RESPONSE" | grep -q "authenticated"; then
        echo "✅ Authentication endpoint accessible"
    else
        echo "⚠️  Auth endpoint accessible (may need login)"
    fi
else
    echo "⚠️  Auth endpoint may require login or not accessible"
fi

# Test market data endpoint (example)
echo -n "  Testing market data endpoint... "
if curl -s $SSL_FLAG --connect-timeout 10 "${GATEWAY_URL}/v1/api/iserver/marketdata/snapshot?conids=265598&fields=31" > /dev/null 2>&1; then
    echo "✅ Market data endpoint accessible"
else
    echo "⚠️  Market data endpoint may require subscription"
fi

echo ""
echo "=========================================="
echo "Connection Test Summary"
echo "=========================================="
echo "Gateway URL: $GATEWAY_URL"
echo "Status: ✅ Connection successful"
echo ""
echo "Next steps:"
echo "1. Ensure you are logged into IBKR Gateway"
echo "2. Check if your account has market data subscriptions"
echo "3. Run the scanner: cargo run"
echo ""
echo "If you encounter authentication issues:"
echo "1. Open IBKR Gateway in your browser: $GATEWAY_URL"
echo "2. Log in with your IBKR credentials"
echo "3. Ensure API access is enabled in gateway settings"
echo ""
echo "To test manually, try:"
echo "  curl $SSL_FLAG '${GATEWAY_URL}/v1/api/tickle'"
echo "  curl $SSL_FLAG '${GATEWAY_URL}/v1/api/iserver/auth/status'"