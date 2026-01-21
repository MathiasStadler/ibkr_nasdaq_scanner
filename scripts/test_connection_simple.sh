#!/bin/bash
# test_connection_simple.sh - Simple IBKR Gateway Connection Test

set -e

echo "🔍 IBKR Gateway Connection Tester"
echo "=================================="

# Check for .env file
if [ ! -f ../.env ]; then
    echo "❌ Error: .env file not found in parent directory"
    echo "Looking for: $(pwd)/../.env"
    exit 1
fi

# Load key environment variables manually
IBKR_GATEWAY_HOST=$(grep -E '^IBKR_GATEWAY_HOST=' ../.env | cut -d '=' -f2 | tr -d '"'"'")
IBKR_GATEWAY_PORT=$(grep -E '^IBKR_GATEWAY_PORT=' ../.env | cut -d '=' -f2 | tr -d '"'"'")
IBKR_GATEWAY_SSL=$(grep -E '^IBKR_GATEWAY_SSL=' ../.env | cut -d '=' -f2 | tr -d '"'"'")

# Set defaults if empty
IBKR_GATEWAY_HOST=${IBKR_GATEWAY_HOST:-localhost}
IBKR_GATEWAY_PORT=${IBKR_GATEWAY_PORT:-5000}
IBKR_GATEWAY_SSL=${IBKR_GATEWAY_SSL:-false}

echo "📋 Configuration:"
echo "  Host: $IBKR_GATEWAY_HOST"
echo "  Port: $IBKR_GATEWAY_PORT"
echo "  SSL: $IBKR_GATEWAY_SSL"
echo ""

# Try HTTP first (most common)
echo "1️⃣ Testing HTTP connection..."
if [[ "$IBKR_GATEWAY_SSL" == "false" ]]; then
    URL="http://${IBKR_GATEWAY_HOST}:${IBKR_GATEWAY_PORT}"
    echo "   Testing: $URL"
    
    if curl -s --connect-timeout 10 "${URL}/v1/api/tickle" > /dev/null; then
        echo "   ✅ HTTP connection successful!"
        echo ""
        echo "✅ Connection test passed!"
        echo "   Gateway is accessible via HTTP"
        exit 0
    else
        echo "   ❌ HTTP connection failed"
    fi
else
    echo "   Skipping (SSL configured)"
fi

# Try HTTPS if configured or if HTTP failed
echo ""
echo "2️⃣ Testing HTTPS connection..."
URL="https://${IBKR_GATEWAY_HOST}:${IBKR_GATEWAY_PORT}"
echo "   Testing: $URL"

# Try with SSL verification disabled (for self-signed certs)
if curl -s --insecure --connect-timeout 10 "${URL}/v1/api/tickle" > /dev/null; then
    echo "   ✅ HTTPS connection successful (with self-signed cert)"
    echo ""
    echo "✅ Connection test passed!"
    echo "   Gateway is accessible via HTTPS (self-signed certificate)"
    exit 0
else
    echo "   ❌ HTTPS connection failed"
fi

# Test raw TCP connection
echo ""
echo "3️⃣ Testing raw TCP connection..."
if nc -z -w 5 "$IBKR_GATEWAY_HOST" "$IBKR_GATEWAY_PORT" 2>/dev/null; then
    echo "   ✅ Port $IBKR_GATEWAY_PORT is open on $IBKR_GATEWAY_HOST"
    echo ""
    echo "⚠️  Gateway is reachable but API may not be responding"
    echo "   Check:"
    echo "   1. Is IBKR Gateway running?"
    echo "   2. Is the correct port configured?"
    echo "   3. Check gateway logs for errors"
else
    echo "   ❌ Cannot connect to port $IBKR_GATEWAY_PORT on $IBKR_GATEWAY_HOST"
    echo ""
    echo "❌ Connection failed completely"
fi

echo ""
echo "🔧 Troubleshooting steps:"
echo "1. Ensure IBKR Gateway is installed and running"
echo "2. Check if gateway is configured for port $IBKR_GATEWAY_PORT"
echo "3. Verify SSL setting in .env matches gateway configuration"
echo "4. Try accessing gateway in browser:"
echo "   - HTTP: http://$IBKR_GATEWAY_HOST:$IBKR_GATEWAY_PORT"
echo "   - HTTPS: https://$IBKR_GATEWAY_HOST:$IBKR_GATEWAY_PORT (ignore SSL warnings)"
echo "5. Check gateway logs for specific error messages"
echo ""
echo "💡 Common solutions:"
echo "- If using SSL but getting 'not an SSL/TLS record':"
echo "  Set IBKR_GATEWAY_SSL=false in .env file"
echo "- If gateway is on different machine:"
echo "  Update IBKR_GATEWAY_HOST to the correct IP/hostname"
echo "- If port is different:"
echo "  Update IBKR_GATEWAY_PORT in .env file"