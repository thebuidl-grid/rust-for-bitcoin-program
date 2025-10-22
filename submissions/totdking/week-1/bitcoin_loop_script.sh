#!/bin/bash

# Bitcoin Loop Script - Runs 3 iterations of Bitcoin operations
# This script demonstrates various Bitcoin CLI commands in a loop

echo "Starting Bitcoin Loop Script - 3 iterations"
echo "============================================="

for i in {1..3}; do
    echo ""
    echo "=== ITERATION $i ==="
    
    bitcoin-cli --regtest -rpcwallet="testwallet" getnewaddress
    
    echo "=== END ITERATION $i ==="
    echo ""
    
    # Add a small delay between iterations
    sleep 1
done

echo "Script completed! All 3 iterations finished."
echo "Final blockchain state:"
bitcoin-cli -regtest getblockchaininfo | jq '.blocks, .chain, .verificationprogress'
echo "addresses controlled by my wallet:"
bitcoin-cli -regtest -rpcwallet="testwallet" listreceivedbyaddress 0 true
