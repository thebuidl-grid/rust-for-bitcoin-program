# Mini Bitcoin Blockchain CLI Simulator

## Overview

Rust CLI simulator for Bitcoin-like blockchain basics: block creation, PoW mining (simplified: 2 leading zero bytes), validation, JSON persistence, and interactive mining. Educational tool for blockchain concepts.

**Features**:
- Interactive mining with custom Merkle roots.
- Step-by-step simulation.
- Auto-save/load from `blockchain.json`.
- Chain validation (hashes, links, PoW).

**Limitations**:
- Fixed low difficulty; no real txns/network.
- Mining timeout: 1M nonces.

## Prerequisites & Dependencies

- Rust 1.70+.
- `Cargo.toml`:
  ```toml
  [dependencies]
  sha2 = "0.10"
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  hex = "0.4"
  ```

## CLI Functions

- `mine_new_block(&mut bc)`: Prompt Merkle; mine/add/save.
- `view_blockchain(&bc)`: Print chain details.
- `validate_blockchain(&bc)`: Run `is_valid`.
- `run_simulation()`: Tutorial with fixed params; demo mine.

## Main Loop

- Init `Blockchain` (easy target); load JSON.
- Menu: 1=Sim, 2=View, 3=Validate, 4=Load, 5=Mine, 6=Exit.
- Pauses post-action; orange styling.

## Build/Run

`cargo build && cargo run`

**Example Mining Output**:
```
🔨 Mining block...
Target: 000000000000ffffff...
Nonce: 12345 | Hash: 0000abcd...
✅ Block mined! Nonce: 12345
✨ Added; 💾 Saved.
```
