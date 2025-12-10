# Bitcoin Script Interpreter

A Rust implementation of a Bitcoin Script interpreter that executes opcodes and validates Pay-to-Public-Key-Hash (P2PKH) transactions with detailed execution logging.

## Features

- **Stack-based execution engine** for Bitcoin scripts
- **Core opcode support**: OP_DUP, OP_HASH160, OP_EQUALVERIFY, OP_CHECKSIG, and more
- **P2PKH transaction validation** with comprehensive testing
- **Detailed execution logging** showing step-by-step script processing
- **Command-line interface** for script execution and transaction validation
- **Integration with bitcoind** regtest mode for real-world testing

## Supported Opcodes

- `OP_DUP` (0x76): Duplicate the top stack item
- `OP_HASH160` (0xa9): SHA-256 followed by RIPEMD-160 hash
- `OP_EQUALVERIFY` (0x88): Verify two values are equal, fail if not
- `OP_CHECKSIG` (0xac): Verify ECDSA signature
- `OP_EQUAL` (0x87): Check if two values are equal
- `OP_VERIFY` (0x69): Verify top stack value is true
- `OP_DROP` (0x75): Remove top stack item
- `OP_SWAP` (0x7c): Swap top two stack items
- `OP_PUSHBYTES_N` (1-75): Push N bytes onto the stack

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)
- bitcoind (optional, for testing with real transactions)

### Building from Source

```bash
cd bitcoin-script-interpreter
cargo build --release
```

### Running Tests

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## Usage

### Execute a Script

Execute a Bitcoin script in hex format:

```bash
cargo run -- execute-script <script_hex> --verbose
```

Example:

```bash
cargo run -- execute-script "01420176" --verbose
```

This pushes the value 0x42 onto the stack and duplicates it.

### Validate a Transaction

Validate a P2PKH transaction:

```bash
cargo run -- validate-transaction <tx_hex> <script_pubkey_hex> --input-index 0 --verbose
```

### Run Test Cases

Run built-in valid P2PKH test cases:

```bash
cargo run -- test-p2pkh --verbose
```

Run invalid P2PKH test cases:

```bash
cargo run -- test-p2pkh --invalid --verbose
```

## Example Output

When running with `--verbose`, you'll see detailed execution logs:

```
╔═══════════════════════════════════════╗
║   BITCOIN SCRIPT VALIDATION           ║
╚═══════════════════════════════════════╝

ScriptSig:    47304402...
ScriptPubKey: 76a914...

┌─────────────────────────────────────┐
│ Phase 1: Execute ScriptSig          │
└─────────────────────────────────────┘

Executing: OP_PUSHBYTES_71
  Pushing 71 bytes: 304402...
  Stack (top to bottom):
    [0]: 304402...

Executing: OP_PUSHBYTES_33
  Pushing 33 bytes: 02b4632d...
  Stack (top to bottom):
    [0]: 02b4632d...
    [1]: 304402...

┌─────────────────────────────────────┐
│ Phase 2: Execute ScriptPubKey       │
└─────────────────────────────────────┘

Executing: OP_DUP
  ...

╔═══════════════════════════════════════╗
║   VALIDATION RESULT                   ║
╚═══════════════════════════════════════╝
Status: ✓ VALID
```

## Testing with bitcoind

### Setup bitcoind in regtest mode

1. Install bitcoind:

```bash
sudo apt-get install bitcoind
```

2. Start bitcoind in regtest mode:

```bash
bitcoind -regtest -daemon
```

3. Create a wallet:

```bash
bitcoin-cli -regtest createwallet testwallet
```

4. Generate blocks to get mature coins:

```bash
bitcoin-cli -regtest -generate 101
```

5. Get a new address:

```bash
ADDRESS=$(bitcoin-cli -regtest getnewaddress)
echo $ADDRESS
```

6. Send coins to create a transaction:

```bash
TXID=$(bitcoin-cli -regtest sendtoaddress $ADDRESS 1.0)
echo $TXID
```

7. Get the raw transaction:

```bash
bitcoin-cli -regtest getrawtransaction $TXID
```

8. Get the scriptPubKey for the address:

```bash
bitcoin-cli -regtest getaddressinfo $ADDRESS | jq -r '.scriptPubKey'
```

9. Validate the transaction:

```bash
cargo run -- validate-transaction <tx_hex> <script_pubkey_hex> --verbose
```

## Project Structure

```
bitcoin-script-interpreter/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # Library exports
│   ├── stack.rs          # Stack implementation
│   ├── opcodes.rs        # Opcode definitions and execution
│   ├── interpreter.rs    # Script interpreter engine
│   ├── transaction.rs    # Transaction parsing and validation
│   └── cli.rs            # Command-line interface
├── tests/
│   └── integration_test.rs  # Integration tests
├── Cargo.toml            # Dependencies
└── README.md             # This file
```

## Architecture

### Stack Module

The `Stack` struct manages the data stack used during script execution. It provides:
- Push/pop operations with error handling
- Peek to view top element without removing
- Duplicate operation
- State inspection and pretty printing

### Opcodes Module

Defines the `Opcode` enum representing Bitcoin opcodes and implements execution logic for each:
- Parses byte values to opcode variants
- Executes opcodes with stack manipulation
- Handles cryptographic operations (hashing, signature verification)

### Interpreter Module

The `Interpreter` struct orchestrates script execution:
- Processes scripts byte by byte
- Maintains execution context (stack, transaction data)
- Provides detailed logging of each step
- Combines scriptSig and scriptPubKey for validation

### Transaction Module

Handles Bitcoin transaction structures:
- Parses transaction hex format
- Computes signature hashes for OP_CHECKSIG
- Validates P2PKH scripts
- Creates and validates script templates

## P2PKH (Pay-to-Public-Key-Hash)

A standard P2PKH script has this structure:

**ScriptPubKey:**
```
OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG
```

**ScriptSig:**
```
<signature> <publicKey>
```

**Combined execution:**
1. Push signature onto stack
2. Push public key onto stack
3. OP_DUP duplicates public key
4. OP_HASH160 hashes the duplicated key
5. Push pubKeyHash onto stack (from scriptPubKey)
6. OP_EQUALVERIFY checks hash matches
7. OP_CHECKSIG verifies signature against public key

## Dependencies

- `bitcoin`: Bitcoin protocol types and utilities
- `secp256k1`: ECDSA signature verification
- `hex`: Hex encoding/decoding
- `sha2`: SHA-256 hashing
- `ripemd`: RIPEMD-160 hashing
- `clap`: Command-line argument parsing
- `anyhow`: Error handling
