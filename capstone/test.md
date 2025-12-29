# Testing Guide

This guide shows you how to test all three projects: the Script Interpreter, Block Explorer API, and the core Blockchain Node.

## Prerequisites

Make sure you have Rust installed:
```bash
rustc --version
cargo --version
```

## Project 1: Bitcoin Script Interpreter Tests

### Test 1: Run the Script Interpreter Binary

```bash
cargo run --bin script_interpreter
```

**Expected Output:**
```
Bitcoin Script Interpreter - Project 1

=====================================

=== Example: Valid P2PKH Script ===

Executing script with 6 opcodes
  [0] OP_PUSHDATA(...)
  [1] OP_PUSHDATA(...)
  [2] OP_DUP
  [3] OP_HASH160
  [4] OP_PUSHDATA(...)
  [5] OP_EQUALVERIFY
  [6] OP_CHECKSIG

✓ P2PKH script executed successfully!

=== Example: Invalid P2PKH Script (Wrong Hash) ===
...
✓ Script correctly failed: OP_EQUALVERIFY: values not equal

=== Additional P2PKH Test Cases ===
...
```

### Test 2: Test Valid P2PKH Script

Create a test file `test_script.rs`:

```rust
use mini_bitcoin_node::{ScriptInterpreter};
use sha2::{Sha256, Digest};
use ripemd160::{Ripemd160, Digest as Ripemd160Digest};

fn main() {
    let interpreter = ScriptInterpreter::new();
    
    // Create public key and calculate its hash
    let public_key = b"test_public_key_32_bytes_long!".to_vec();
    
    let mut sha256 = Sha256::new();
    sha256.update(&public_key);
    let sha256_hash = sha256.finalize();
    
    let mut ripemd160 = Ripemd160::new();
    ripemd160.update(&sha256_hash);
    let pubkey_hash = ripemd160.finalize().to_vec();
    
    let signature = b"valid_signature_data".to_vec();
    
    // Test valid P2PKH
    match interpreter.execute_p2pkh(&signature, &public_key, &pubkey_hash) {
        Ok(true) => println!("✓ Test PASSED: Valid P2PKH accepted"),
        _ => println!("✗ Test FAILED: Valid P2PKH rejected"),
    }
}
```

Run it:
```bash
cargo run --example test_script  # If you create it as an example
```

### Test 3: Test Invalid P2PKH Script

```rust
// Wrong pubkey hash
let wrong_hash = b"wrong_hash_20_bytes!!".to_vec();

match interpreter.execute_p2pkh(&signature, &public_key, &wrong_hash) {
    Ok(true) => println!("✗ Test FAILED: Invalid P2PKH accepted"),
    Ok(false) => println!("✓ Test PASSED: Invalid P2PKH rejected (false)"),
    Err(_) => println!("✓ Test PASSED: Invalid P2PKH rejected (error)"),
}
```

### Test 4: Test Individual Opcodes

```rust
use mini_bitcoin_node::{ScriptInterpreter, ScriptContext, Opcode};

let interpreter = ScriptInterpreter::new();
let mut context = ScriptContext::new();

// Test OP_DUP
context.push(b"test".to_vec());
interpreter.execute(&[Opcode::OP_DUP], &mut context)?;
assert_eq!(context.stack.len(), 2); // Should have duplicated item
println!("✓ OP_DUP test passed");

// Test OP_HASH160
context = ScriptContext::new();
context.push(b"hello".to_vec());
interpreter.execute(&[Opcode::OP_HASH160], &mut context)?;
assert_eq!(context.stack[0].len(), 20); // RIPEMD160 produces 20 bytes
println!("✓ OP_HASH160 test passed");
```

## Project 2: Block Explorer API Tests

### Test 1: Start the API Server

```bash
# Terminal 1: Start the server
cargo run --bin block_explorer_api
```

**Expected Output:**
```
Bitcoin Block Explorer API - Project 2

======================================

Opening database: blockchain.db
Database is empty. Creating sample blockchain data...
Mining block 1...
Block mined! Hash: ...
Mining block 2...
...
Database initialized. Height: 3

API server listening on http://0.0.0.0:3000
Endpoints:
  GET /health
  GET /block/:hash
  GET /tx/:txid
```

### Test 2: Health Check Endpoint

```bash
# Terminal 2: Test health endpoint
curl http://localhost:3000/health
```

**Expected Output:**
```json
{
  "status": "ok",
  "service": "bitcoin-block-explorer"
}
```

### Test 3: Get Block by Hash

First, get a block hash from the server logs, then:

```bash
# Replace <hash> with actual block hash from logs
curl http://localhost:3000/block/<hash>
```

**Expected Output:**
```json
{
  "hash": "abc123...",
  "prev_hash": "def456...",
  "height": 1,
  "timestamp": 1234567890,
  "merkle_root": "789ghi...",
  "nonce": 12345,
  "bits": 4,
  "transaction_count": 2
}
```

### Test 4: Get Transaction by ID

```bash
# Get a transaction ID from a block, then:
curl http://localhost:3000/tx/<txid>
```

**Expected Output:**
```json
{
  "txid": "xyz789...",
  "block_hash": "abc123...",
  "block_height": 1,
  "timestamp": 1234567890,
  "is_coinbase": false,
  "input_count": 1,
  "output_count": 2
}
```

### Test 5: Test Non-Existent Block

```bash
curl http://localhost:3000/block/nonexistent
```

**Expected Output:**
```json
{
  "error": "Block not found: nonexistent"
}
```
HTTP Status: 404

### Test 6: Test Non-Existent Transaction

```bash
curl http://localhost:3000/tx/nonexistent
```

**Expected Output:**
```json
{
  "error": "Transaction not found: nonexistent"
}
```
HTTP Status: 404

### Test 7: Test with Custom Port

```bash
PORT=8080 cargo run --bin block_explorer_api
# Then test:
curl http://localhost:8080/health
```

### Test 8: Test with Custom Database

```bash
DB_PATH=test_blockchain.db cargo run --bin block_explorer_api
# Check that test_blockchain.db file is created
ls -la test_blockchain.db
```

## Core Blockchain Node Tests

### Test 1: Run the Main Demo

```bash
cargo run
```

**Expected Output:**
```
=== Mini Bitcoin Node Simulator ===

1. Created new node with genesis block
   Blockchain height: 1
   Latest block hash: ...

✓ Blockchain is valid

2. Creating transactions...
   Created transaction 1: ... -> address1 (30 BTC)
   Created transaction 2: address1 -> address2 (20 BTC)

3. Submitting transactions to mempool...
   ✓ Transaction 1 added to mempool
   ✓ Transaction 2 added to mempool
   Pending transactions: 2

4. Mining block with pending transactions...
Mining block...
Block mined! Hash: ...
   ✓ Block mined successfully!
   Block hash: ...
   Transactions in block: 3
   Nonce: ...

5. Checking balances...
   Genesis balance: ... satoshis
   Address1 balance: ... satoshis
   Address2 balance: ... satoshis
   Miner1 balance: ... satoshis

6. Validating blockchain after mining...
   ✓ Blockchain is still valid

7. Testing double-spend prevention...
   ✓ Double-spend prevented: ...

8. Testing insufficient funds prevention...
   ✓ Insufficient funds prevented: ...

9. Node Information:
   Height: 2
   Latest hash: ...
   Pending transactions: 0
   Difficulty: 4

=== Demo Complete ===
```

### Test 2: Test Transaction Validation

Create a test file:

```rust
use mini_bitcoin_node::{Transaction, TxInput, TxOutput};
use std::collections::HashMap;

// Test valid transaction
let mut utxo_set = HashMap::new();
utxo_set.insert("prev_tx".to_string(), vec![
    TxOutput { amount: 1000, address: "addr1".to_string() }
]);

let tx = Transaction::new(
    vec![TxInput {
        prev_tx_id: "prev_tx".to_string(),
        output_index: 0,
        signature: "sig".to_string(),
    }],
    vec![TxOutput {
        amount: 500,
        address: "addr2".to_string(),
    }],
);

match tx.is_valid(&utxo_set) {
    Ok(()) => println!("✓ Valid transaction accepted"),
    Err(e) => println!("✗ Valid transaction rejected: {}", e),
}
```

### Test 3: Test Double-Spend Prevention

```rust
// Try to spend same UTXO twice
let tx1 = Transaction::new(/* ... */);
let tx2 = Transaction::new(/* same input */);

node.submit_transaction(tx1)?;
match node.submit_transaction(tx2) {
    Ok(()) => println!("✗ Double-spend allowed (BUG!)"),
    Err(_) => println!("✓ Double-spend prevented"),
}
```

### Test 4: Test Insufficient Funds

```rust
// Try to spend more than available
let tx = Transaction::new(
    vec![TxInput {
        prev_tx_id: "prev_tx".to_string(),
        output_index: 0, // Has 1000 satoshis
        signature: "sig".to_string(),
    }],
    vec![TxOutput {
        amount: 2000, // Trying to spend 2000
        address: "addr".to_string(),
    }],
);

match tx.is_valid(&utxo_set) {
    Ok(()) => println!("✗ Insufficient funds allowed (BUG!)"),
    Err(_) => println!("✓ Insufficient funds prevented"),
}
```

## Integration Tests

### Test 1: Full Workflow Test

```bash
# 1. Create blockchain and mine blocks
cargo run  # Creates blockchain

# 2. Start API server (in another terminal)
cargo run --bin block_explorer_api

# 3. Query blocks via API
curl http://localhost:3000/block/<hash>

# 4. Test script interpreter
cargo run --bin script_interpreter
```

### Test 2: Database Persistence Test

```bash
# 1. Start API server (creates database)
cargo run --bin block_explorer_api

# 2. Stop server (Ctrl+C)

# 3. Start again - should load existing data
cargo run --bin block_explorer_api

# Should show: "Database initialized. Height: 3" (not 0)
```

### Test 3: Script Validation in Transactions

```rust
use mini_bitcoin_node::*;

// Create transaction
let tx = Transaction::new(/* ... */);

// Validate with script interpreter
let interpreter = ScriptInterpreter::new();
// ... validate transaction outputs with scripts
```

## Automated Test Script

Create `test_all.sh`:

```bash
#!/bin/bash

echo "=== Testing Bitcoin Node Projects ==="

echo ""
echo "1. Testing Script Interpreter..."
cargo run --bin script_interpreter > /tmp/script_test.log 2>&1
if [ $? -eq 0 ]; then
    echo "✓ Script Interpreter tests passed"
else
    echo "✗ Script Interpreter tests failed"
    cat /tmp/script_test.log
fi

echo ""
echo "2. Testing Core Blockchain..."
cargo run > /tmp/blockchain_test.log 2>&1
if [ $? -eq 0 ]; then
    echo "✓ Blockchain tests passed"
else
    echo "✗ Blockchain tests failed"
    cat /tmp/blockchain_test.log
fi

echo ""
echo "3. Testing Block Explorer API..."
# Start server in background
cargo run --bin block_explorer_api > /tmp/api_test.log 2>&1 &
API_PID=$!
sleep 3

# Test health endpoint
if curl -s http://localhost:3000/health | grep -q "ok"; then
    echo "✓ API health check passed"
else
    echo "✗ API health check failed"
fi

# Stop server
kill $API_PID

echo ""
echo "=== All Tests Complete ==="
```

Make it executable and run:
```bash
chmod +x test_all.sh
./test_all.sh
```

## Manual Testing Checklist

### Script Interpreter (Project 1)
- [ ] Run `cargo run --bin script_interpreter`
- [ ] Verify valid P2PKH script succeeds
- [ ] Verify invalid P2PKH script fails
- [ ] Check execution logging output
- [ ] Test individual opcodes (OP_DUP, OP_HASH160, etc.)

### Block Explorer API (Project 2)
- [ ] Start API server: `cargo run --bin block_explorer_api`
- [ ] Test `/health` endpoint returns 200
- [ ] Test `/block/:hash` with valid hash returns block data
- [ ] Test `/block/:hash` with invalid hash returns 404
- [ ] Test `/tx/:txid` with valid txid returns transaction
- [ ] Test `/tx/:txid` with invalid txid returns 404
- [ ] Verify database persists between restarts
- [ ] Test custom port with `PORT=8080`

### Core Blockchain Node
- [ ] Run `cargo run` and verify all steps complete
- [ ] Verify genesis block is created
- [ ] Verify transactions are validated
- [ ] Verify blocks are mined successfully
- [ ] Verify double-spend is prevented
- [ ] Verify insufficient funds is prevented
- [ ] Verify blockchain validation passes

## Expected Test Results

### Script Interpreter
- ✅ Valid P2PKH: Should return `Ok(true)`
- ✅ Invalid P2PKH: Should return `Ok(false)` or `Err(...)`
- ✅ OP_DUP: Should duplicate top stack item
- ✅ OP_HASH160: Should produce 20-byte hash
- ✅ OP_EQUALVERIFY: Should verify equality or error

### Block Explorer API
- ✅ `/health`: Should return `{"status":"ok"}`
- ✅ `/block/:hash`: Should return block JSON or 404
- ✅ `/tx/:txid`: Should return transaction JSON or 404
- ✅ Database: Should persist data between restarts

### Core Blockchain
- ✅ Genesis block: Should be created automatically
- ✅ Transactions: Should validate correctly
- ✅ Blocks: Should mine with valid proof-of-work
- ✅ Double-spend: Should be rejected
- ✅ Insufficient funds: Should be rejected
- ✅ Chain validation: Should pass for valid chain

## Troubleshooting Tests

### Issue: Script Interpreter fails to compile
```bash
# Check dependencies
cargo check
# Update dependencies
cargo update
```

### Issue: API server won't start
```bash
# Check if port is in use
lsof -i :3000
# Use different port
PORT=8080 cargo run --bin block_explorer_api
```

### Issue: Database locked errors
```bash
# Make sure only one process uses database
# Close any other API servers
pkill -f block_explorer_api
```

### Issue: Tests fail unexpectedly
```bash
# Clean and rebuild
cargo clean
cargo build
cargo test  # If you add unit tests
```

## Next Steps

After running these tests:
1. Review the output logs
2. Verify all expected behaviors
3. Try creating your own test cases
4. Experiment with different script patterns
5. Test API with different block/transaction data

Happy testing! 🚀

