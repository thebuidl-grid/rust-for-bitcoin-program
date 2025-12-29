# Quick Start Guide

## Project 1: Bitcoin Script Interpreter

### Run the Script Interpreter

```bash
cargo run --bin script_interpreter
```

This will demonstrate:
- ✓ Valid P2PKH script execution
- ✗ Invalid P2PKH (wrong hash)
- Custom script examples
- Stack operations with logging

### Use in Your Code

```rust
use mini_bitcoin_node::{ScriptInterpreter, ScriptContext, Opcode};

// Create interpreter
let interpreter = ScriptInterpreter::new();

// Parse script from hex
let opcodes = ScriptInterpreter::parse_script("76a914...")?;

// Execute script
let mut context = ScriptContext::new();
let result = interpreter.execute(&opcodes, &mut context)?;

// P2PKH validation
let valid = interpreter.execute_p2pkh(&sig, &pubkey, &pubkey_hash)?;
```

## Project 2: Block Explorer API

### Start the Server

```bash
# Default port 3000
cargo run --bin block_explorer_api

# Custom port
PORT=8080 cargo run --bin block_explorer_api

# Custom database
DB_PATH=my_blockchain.db cargo run --bin block_explorer_api
```

### Test the API

```bash
# Health check
curl http://localhost:3000/health

# Get block by hash (use a hash from your blockchain)
curl http://localhost:3000/block/abc123...

# Get transaction by ID
curl http://localhost:3000/tx/def456...
```

### Use in Your Code

```rust
use mini_bitcoin_node::{BlockDatabase, start_server};

// Create database
let db = BlockDatabase::new("blockchain.db")?;

// Index blocks
for (height, block) in blocks.iter().enumerate() {
    db.add_block(block, height)?;
}

// Start API server
start_server(db, 3000).await?;
```

## Core Blockchain Node

### Run the Demo

```bash
cargo run
```

### Use in Your Code

```rust
use mini_bitcoin_node::{Node, Transaction, TxInput, TxOutput};

// Create node
let mut node = Node::new("miner_address".to_string());

// Create transaction
let tx = Transaction::new(
    vec![TxInput { /* ... */ }],
    vec![TxOutput { /* ... */ }],
);

// Submit to mempool
node.submit_transaction(tx)?;

// Mine block
let block = node.mine_block()?;

// Check balance
let balance = node.get_balance("address");
```

## Integration Example

Combine all three projects:

```rust
use mini_bitcoin_node::*;

// 1. Create blockchain node
let mut node = Node::new("miner".to_string());

// 2. Mine some blocks
for _ in 0..5 {
    node.mine_block()?;
}

// 3. Index into database
let db = BlockDatabase::new("blockchain.db")?;
for (height, block) in node.blockchain.blocks.iter().enumerate() {
    db.add_block(block, height)?;
}

// 4. Start API server
start_server(db, 3000).await?;

// 5. Validate scripts
let interpreter = ScriptInterpreter::new();
// ... validate transactions with scripts
```

## Troubleshooting

### Database locked errors
- Make sure only one process accesses the database at a time
- Close database connections properly

### Script execution fails
- Check that script hex is valid
- Verify opcodes are supported
- Check stack has enough items

### API not responding
- Check port is not in use: `lsof -i :3000`
- Verify database exists and has data
- Check logs for errors

