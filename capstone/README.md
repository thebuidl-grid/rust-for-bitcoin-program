# Rust Bitcoin Script & Transaction Toolkit

A combined capstone project for the Rust for Bitcoin Program, implementing:

- A fully functional Bitcoin Script Interpreter
- A complete Raw Transaction Builder + Signing Engine
- A CLI tool (btc_tx) for building, signing, and verifying Bitcoin transactions

This project demonstrates low-level mastery of Bitcoin internals, Rust engineering, and cryptographic primitives.

## 🚀 Features

### 🔸 Bitcoin Script Interpreter

- Stack-based execution engine
- Supported opcodes:
  - OP_DUP
  - OP_HASH160
  - OP_EQUALVERIFY
  - OP_CHECKSIG
- P2PKH script execution flow
- Script context handling
- Signature validation using secp256k1
- Clean modular architecture

### 🔸 Raw Transaction Builder

- Create unsigned transactions
- Add inputs/outputs
- Serialize & parse Bitcoin transactions
- Generate SIGHASH (BIP143 & legacy modes)
- Produce valid ECDSA signatures
- Insert signatures into scriptSig or witness

### 🔸 CLI Tool (btc_tx)

**Commands:**
- `btc_tx build`
- `btc_tx sign`
- `btc_tx verify`

Allows a full workflow:
**Build → Sign → Verify → Export raw hex**

## 📁 Project Structure

```
src/
  script/
    mod.rs
    opcodes.rs
    interpreter.rs
    context.rs
  tx/
    mod.rs
    builder.rs
    signing.rs
    sighash.rs
    parser.rs
  bin/
    btc_tx.rs
  old_modules/        # (archived previous versions)
```

## 🧠 Architecture Diagram

```
                   +---------------------------+
                   |     CLI (btc_tx.rs)       |
                   +-------------+-------------+
                                 |
                                 v
         +-----------------------+-------------------------+
         |                                                   |
         v                                                   v
+--------------------+                          +-----------------------+
| Transaction Builder | <-- depends on --------- | Script Interpreter    |
|  - builder.rs       |                          |  - interpreter.rs     |
|  - parser.rs        |                          |  - opcodes.rs         |
|  - signing.rs       |                          |  - context.rs            |
|  - sighash.rs       |                          |                       |
+--------------------+                          +-----------------------+
```

## 🛠️ Usage

### 1. Build a raw unsigned transaction

```bash
cargo run --bin btc-tx build-tx \
  --from-tx <txid> \
  --output-index <vout> \
  --to <pubkey_hash> \
  --amount <amount> \
  --output raw_unsigned_tx.hex
```

### 2. Sign the transaction

```bash
cargo run --bin btc-tx sign-tx \
  --tx-file raw_unsigned_tx.hex \
  --privkey <private_key_hex> \
  --from-tx <txid> \
  --output-index <vout> \
  --output raw_signed_tx.hex
```

### 3. Verify the transaction with your Script Interpreter

```bash
cargo run --bin btc-tx verify-tx \
  --tx-file raw_signed_tx.hex \
  --pubkey-hash <pubkey_hash_hex>
```

**Expected output:**
```
Transaction VALID ✔
```

### Complete Workflow

For a complete demonstration of the integration:

```bash
cargo run --bin btc-tx workflow
```

This runs the full cycle: Generate keys → Build → Sign → Verify

## 📦 Installation

```bash
git clone <your-fork>
cd <project>
cargo build --release
```

## 🧪 Testing

```bash
cargo test
```

## 🎯 Capstone Requirements Satisfied

- ✔ Bitcoin Script Interpreter
- ✔ Script opcodes: DUP, HASH160, EQUALVERIFY, CHECKSIG
- ✔ Raw transaction construction
- ✔ SIGHASH implementation
- ✔ Transaction signing
- ✔ Interpreter-based validation
- ✔ CLI with clear workflow
- ✔ Clean modular Rust architecture

## 📚 Technical Details

### Script Interpreter

The interpreter implements a complete stack-based execution engine that:
- Parses scripts from hex format
- Executes opcodes on a stack
- Verifies ECDSA signatures using secp256k1
- Validates P2PKH scripts end-to-end

### Transaction Builder

The transaction builder:
- Creates raw Bitcoin transaction format
- Computes SIGHASH for signing
- Generates ECDSA signatures
- Creates scriptSig with signatures and pubkeys

### Integration

The key integration point is the `verify-tx` command, which:
1. Parses the signed transaction
2. Extracts signature and pubkey from scriptSig
3. Computes SIGHASH from transaction structure
4. **Uses the Script Interpreter to validate** the transaction

This demonstrates that transactions built by the Transaction Builder are validated by the Script Interpreter.

## 🔧 Dependencies

- `secp256k1` - ECDSA signature operations
- `sha2` - SHA-256 hashing
- `ripemd160` - RIPEMD-160 hashing
- `hex` - Hex encoding/decoding
- `clap` - CLI argument parsing
- `rand` - Random number generation

## 📝 License

Educational project for learning Bitcoin script execution and transaction building.
