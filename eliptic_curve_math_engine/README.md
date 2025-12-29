# Elliptic Curve Math Engine

A Bitcoin secp256k1 keypair generation and management CLI tool built in Rust.

## Features

- **256-bit modular arithmetic** for secp256k1 curve operations
- **Keypair generation** with cryptographically secure random number generation
- **Public key derivation** from private keys using scalar multiplication
- **Multiple serialization formats**: compressed (33 bytes), uncompressed (65 bytes), and x-only (32 bytes for Taproot/BIP340)
- **Jacobian coordinate system** for efficient elliptic curve operations

## Installation

### Build from Source

```bash
cargo build --release
```

The binary will be created at `./target/release/sec`

### Install Globally (Optional)

To use `sec` from anywhere on your system:

```bash
# Install to ~/.cargo/bin (make sure it's in your PATH)
cargo install --path .

# Or copy the binary to a directory in your PATH
sudo cp ./target/release/sec /usr/local/bin/
```

After installation, you can use `sec` directly instead of `./target/release/sec`

## Usage

The CLI provides three main commands: `generate`, `derive`, and `info`.

### 1. Generate a New Keypair

Generate a new random keypair with the default compressed public key format:

```bash
sec generate
```

**Example Output:**
```
Public Key (compressed): 02a1b2c3d4e5f6...
```

#### Options:

- **Show private key** (⚠️ WARNING: Only for development/testing):
  ```bash
  sec generate --show-private
  ```

- **Specify output format**:
  ```bash
  # Compressed format (33 bytes) - default
  sec generate --format compressed
  
  # Uncompressed format (65 bytes)
  sec generate --format uncompressed
  
  # X-only format (32 bytes) - for Taproot/BIP340
  sec generate --format x-only
  ```

- **Combined options**:
  ```bash
  sec generate --format uncompressed --show-private
  ```

### 2. Derive Public Key from Private Key

Derive a public key from an existing private key (in hexadecimal format):

```bash
sec derive --private-key <HEX_PRIVATE_KEY>
```

**Example:**
```bash
sec derive --private-key 1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
```

**Example Output:**
```
Public Key (compressed): 02a1b2c3d4e5f6...
```

#### Options:

- **Specify output format**:
  ```bash
  sec derive --private-key <HEX> --format compressed
  sec derive --private-key <HEX> --format uncompressed
  sec derive --private-key <HEX> --format x-only
  ```

### 3. Display Keypair Information

Display all public key formats for a given private key:

```bash
sec info --private-key <HEX_PRIVATE_KEY>
```

**Example Output:**
```
=== Keypair Information ===
Private Key: 1234567890abcdef...

Public Key Formats:
  Compressed (33 bytes):   02a1b2c3d4e5f6...
  Uncompressed (65 bytes): 04a1b2c3d4e5f6...
  X-Only (32 bytes):       a1b2c3d4e5f6...
```

## Command Reference

### Global Options

- `--help` - Display help information
- `--version` - Display version information

### Commands

| Command | Description | Required Args | Optional Args |
|---------|-------------|---------------|---------------|
| `generate` | Generate a new keypair | None | `--format`, `--show-private` |
| `derive` | Derive public key from private key | `--private-key` | `--format` |
| `info` | Display all keypair information | `--private-key` | None |

### Format Options

- `compressed` - 33-byte compressed format (prefix: 0x02 or 0x03)
- `uncompressed` - 65-byte uncompressed format (prefix: 0x04)
- `x-only` - 32-byte x-coordinate only (for Taproot/BIP340)

## Examples

### Generate and Save a Keypair

```bash
# Generate keypair and save to file
sec generate --show-private > my_keypair.txt
```

### Derive Public Key in All Formats

```bash
# Show all formats for a private key
sec info --private-key abc123def456...
```

### Quick Public Key Lookup

```bash
# Get compressed public key from private key
sec derive -p abc123def456... -f compressed
```

## Security Notes

⚠️ **IMPORTANT SECURITY WARNINGS:**

1. **Never share your private key** - Anyone with your private key has full control of your funds
2. **The `--show-private` flag is for development only** - Never use in production
3. **Private keys are 256-bit random numbers** - Must be within range `0 < k < n` where `n` is the curve order
4. **Store private keys securely** - Use hardware wallets or encrypted storage in production

## Technical Details

### Elliptic Curve Operations

- **Curve**: secp256k1 (y² = x³ + 7)
- **Field Prime (P)**: 2²⁵⁶ - 2³² - 977
- **Curve Order (N)**: FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141
- **Generator Point (G)**: Standard secp256k1 generator

### Implementation

- **Coordinate System**: Jacobian coordinates for efficient computation
- **Scalar Multiplication**: Double-and-add algorithm
- **Random Generation**: OS-provided cryptographically secure RNG
- **Modular Arithmetic**: Custom 256-bit field element implementation

## Running Tests

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run specific test module
cargo test jacobian_test
cargo test ec_point::tests

# Run with output
cargo test -- --nocapture
```

## Development

### Project Structure

```
src/
├── cli/
│   ├── handler.rs      # CLI command handlers
│   └── mod.rs
├── keypair_deriv/
│   ├── keypair.rs      # Keypair generation
│   ├── private_key.rs  # Private key wrapper
│   └── pubkey.rs       # Public key wrapper
├── point_arithmetic/
│   ├── ec_point.rs     # Affine coordinates & serialization
│   ├── field_element.rs # Modular arithmetic
│   └── jacobian_point.rs # Jacobian coordinates & operations
├── lib.rs
└── main.rs             # CLI entry point
```

## License

This project is for educational purposes.