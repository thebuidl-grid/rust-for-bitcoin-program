#!/bin/bash

echo "=== Testing Bitcoin Node Projects ==="
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASSED=0
FAILED=0

# Test 1: Script Interpreter
echo -e "${YELLOW}1. Testing Script Interpreter...${NC}"
if cargo run --bin script_interpreter > /tmp/script_test.log 2>&1; then
    echo -e "${GREEN}✓ Script Interpreter tests passed${NC}"
    PASSED=$((PASSED + 1))
else
    echo -e "${RED}✗ Script Interpreter tests failed${NC}"
    echo "Last 20 lines of output:"
    tail -20 /tmp/script_test.log
    FAILED=$((FAILED + 1))
fi

echo ""

# Test 2: Core Blockchain
echo -e "${YELLOW}2. Testing Core Blockchain...${NC}"
if cargo run > /tmp/blockchain_test.log 2>&1; then
    echo -e "${GREEN}✓ Blockchain tests passed${NC}"
    PASSED=$((PASSED + 1))
else
    echo -e "${RED}✗ Blockchain tests failed${NC}"
    echo "Last 20 lines of output:"
    tail -20 /tmp/blockchain_test.log
    FAILED=$((FAILED + 1))
fi

echo ""

# Test 3: Block Explorer API
echo -e "${YELLOW}3. Testing Block Explorer API...${NC}"

# Clean up any existing database
rm -f /tmp/test_blockchain.db

# Start server in background with custom database path
DB_PATH=/tmp/test_blockchain.db cargo run --bin block_explorer_api > /tmp/api_test.log 2>&1 &
API_PID=$!

# Wait for server to start
sleep 5

# Test health endpoint
if curl -s http://localhost:3000/health 2>/dev/null | grep -q "ok"; then
    echo -e "${GREEN}✓ API health check passed${NC}"
    PASSED=$((PASSED + 1))
    
    # Try to get a block (might fail if no blocks, but server should respond)
    BLOCK_RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:3000/block/test123 2>/dev/null)
    if [ "$BLOCK_RESPONSE" = "404" ] || [ "$BLOCK_RESPONSE" = "200" ]; then
        echo -e "${GREEN}✓ API block endpoint responding${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}✗ API block endpoint not responding correctly${NC}"
        FAILED=$((FAILED + 1))
    fi
else
    echo -e "${RED}✗ API health check failed${NC}"
    echo "Server log:"
    tail -10 /tmp/api_test.log
    FAILED=$((FAILED + 1))
fi

# Stop server
kill $API_PID 2>/dev/null
wait $API_PID 2>/dev/null

echo ""

# Summary
echo "=== Test Summary ==="
echo -e "${GREEN}Passed: $PASSED${NC}"
echo -e "${RED}Failed: $FAILED${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed! ✓${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed. Check logs above.${NC}"
    exit 1
fi

