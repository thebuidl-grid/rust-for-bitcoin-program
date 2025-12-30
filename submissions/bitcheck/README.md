# bit_check

Bitcoin script validator in Rust. Runs locking and unlocking scripts through a stack interpreter.

## Setup

```toml
[package]
name = "bit_check"
version = "0.1.0"
edition = "2021"

[dependencies]
sha2 = "0.10"
ripemd = "0.1"
hex = "0.4"
```

```bash
cargo run
```

## Usage

Input locking script (scriptPubKey) and unlocking script (scriptSig). Accepts hex or assembly format.

```
Locking Script: OP_DUP OP_HASH160 6291ad5107bf1ab687dc744cc9d082aa9522eff0 OP_EQUALVERIFY OP_CHECKSIG
Type: P2PKH

Unlocking Script: 3045022100... 03d19433...

Run this script? (y/n): y

This is a valid script!
```

## Features

- Parses hex and assembly scripts
- Detects P2PK, P2PKH, P2SH, P2MS, OP_RETURN
- Stack-based execution
- Hash160 (SHA256 + RIPEMD160)

## Opcodes

OP_DUP, OP_HASH160, OP_EQUAL, OP_EQUALVERIFY, OP_CHECKSIG, OP_CHECKMULTISIG, OP_RETURN, OP_0-16