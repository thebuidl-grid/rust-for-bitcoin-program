# Bitcoin-do

A CLI tool and Rust library for Bitcoin-related cryptography operations.

## Features

- **SHA256**: Compute SHA256 hash of input data
- **RIPEMD160**: Compute RIPEMD160 hash of input data
- **HASH160**: Compute HASH160 (RIPEMD160 of SHA256) of input data
- **Key Generation**: Generate secp256k1 keypairs
- **Address Derivation**: Derive P2PKH Bitcoin addresses from public keys

## Installation

### As a CLI Tool

```bash
cargo install bitcoin-do
```

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
bitcoin-do = "0.1.2"
```

## Usage

### CLI Examples

Generate a new keypair:

```bash
bitcoin-do generate-key
```

Derive address from public key:

```bash
bitcoin-do derive-address 02...
```

Hash some data (hex input):

```bash
bitcoin-do sha256 68656c6c6f
```

```bash
bitcoin-do Hash160 68656c6c6f
```

```bash
bitcoin-do Ripemd160 68656c6c6f
```

## License

MIT
