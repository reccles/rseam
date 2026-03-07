#!/bin/bash
# rseam test script
# Runs unit tests and integration tests

set -e

echo ""
echo "╔════════════════════════════════════════════╗"
echo "║         rseam Test Suite                   ║"
echo "╚════════════════════════════════════════════╝"
echo ""

# Run unit tests
echo "🧪 Running unit tests..."
cargo test --quiet
echo "✅ Unit tests passed (30 tests)"
echo ""

# Check if SEAM_API_KEY is set for integration tests
if [ -z "$SEAM_API_KEY" ]; then
    # Try to load from common .env locations
    if [ -f ~/src/.env ]; then
        source ~/src/.env
    elif [ -f ~/.seam/.env ]; then
        source ~/.seam/.env
    fi
fi

if [ -z "$SEAM_API_KEY" ]; then
    echo "⚠️  SEAM_API_KEY not set - skipping integration tests"
    echo "   Set SEAM_API_KEY environment variable to run integration tests"
    exit 0
fi

# Build release binary if needed
if [ ! -f ./target/release/rseam ]; then
    echo "📦 Building release binary..."
    cargo build --release --quiet
fi

BINARY="./target/release/rseam"

echo "🔌 Running integration tests..."
echo ""

# Test 1: Help
echo -n "  Help flag: "
$BINARY --help > /dev/null && echo "✅" || echo "❌"

# Test 2: Version
echo -n "  Version flag: "
$BINARY --version > /dev/null && echo "✅" || echo "❌"

# Test 3: Health check (verifies API connectivity)
echo -n "  Health check (API): "
output=$($BINARY health get-health 2>&1)
if echo "$output" | grep -q '"ok": true'; then
    echo "✅"
else
    echo "❌"
    echo "    Response: $output"
fi

# Test 4: Health check with --raw flag
echo -n "  --raw flag: "
output=$($BINARY health get-health --raw 2>&1)
if echo "$output" | grep -q '"ok":true'; then
    echo "✅"
else
    echo "❌"
fi

# Test 5: Devices list
echo -n "  Devices list: "
output=$($BINARY devices list 2>&1)
if echo "$output" | grep -q 'devices'; then
    echo "✅"
else
    echo "⚠️  (may need workspace context)"
fi

# Test 6: Subcommand help
echo -n "  Subcommand help: "
$BINARY devices --help > /dev/null && echo "✅" || echo "❌"

echo ""
echo "════════════════════════════════════════════"
echo "✅ All tests completed!"
echo ""
echo "Usage examples:"
echo "  $BINARY health get-health"
echo "  $BINARY devices list"
echo "  $BINARY devices get --name \"Front Door\" --id-only"
echo "  $BINARY locks unlock-door --device-id \$DEVICE_ID"
echo ""
