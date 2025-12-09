# Regtest Block Explorer CLI

A command-line tool for indexing and exploring Bitcoin regtest blocks. It allows users to fetch blocks from a running Bitcoin regtest node via RPC or parse them from local `.blk` files, store them in a SQLite database, and serve block/transaction data via a RESTful web API.

## Table of Contents

- [Features](#features)
- [How It Works](#how-it-works)
- [Architecture](#architecture)
- [Project Structure](#project-structure)
- [Installation](#installation)
- [Usage](#usage)
- [API Endpoints](#api-endpoints)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Dual Indexing Modes**: Index blocks from a live Bitcoin regtest node via RPC or from local `.blk` files.
- **SQLite Database**: Efficient storage and querying of block and transaction data.
- **Web API**: RESTful endpoints to query blocks, transactions, and statistics.
- **CLI Interface**: Simple command-line commands for indexing and serving.
- **Regtest Focused**: Designed for Bitcoin's regtest network (test environment).
- **Pagination Support**: Efficiently browse large sets of blocks.
- **Health Checks**: Built-in endpoint for monitoring service status.

## How It Works

1. **Indexing**: The tool connects to a Bitcoin regtest node (or reads local files) to fetch block data. Each block is parsed, and its details (hash, height, transactions) are stored in a local SQLite database.
2. **Storage**: Uses Rusqlite for database operations. Blocks and transactions are serialized and stored for fast retrieval.
3. **Serving**: An Actix-Web server provides HTTP endpoints to query the indexed data. Responses are in JSON format.
4. **CLI Commands**: Users run commands like `bitcoin-explore index` to populate the database, then `bitcoin-explore serve` to start the API server.

This tool is ideal for developers testing Bitcoin applications or exploring blockchain data without needing a full mainnet node.

## Architecture

- **Language**: Rust (safe, fast, concurrent).
- **Web Framework**: Actix-Web for async HTTP handling.
- **Database**: SQLite via Rusqlite for lightweight, file-based storage.
- **Bitcoin Library**: `bitcoin` crate for parsing and handling Bitcoin data structures.
- **CLI Parsing**: `clap` for command-line argument handling.
- **Async Runtime**: Tokio for asynchronous operations (e.g., file I/O, RPC calls).

The architecture is modular: separate modules for database operations, data models, web handlers, and block parsing. This keeps the code organized and testable.

## Project Structure

```
bitcoin-explore/
├── Cargo.toml          # Project metadata, dependencies, and build configuration
├── README.md           # This file
└── src/
    ├── main.rs         # CLI entry point: defines commands and starts server
    ├── lib.rs          # Module declarations (db, models, parser, handlers)
    ├── db.rs           # Database functions: init, insert, query operations
    ├── models.rs       # Data structures: BlockResponse, TxResponse, etc.
    ├── handlers.rs     # Web API handlers: functions for each endpoint
    └── parser.rs       # Block parsing: reads .blk files and extracts data
```

### File Descriptions

- **`Cargo.toml`**: Defines the project name (`bitcoin-explore`), version, dependencies (e.g., `actix-web`, `bitcoin`, `rusqlite`), and metadata for crates.io publishing.
- **`src/main.rs`**: The main binary. Uses `clap` to parse CLI arguments into subcommands (`index`, `serve`). Handles RPC indexing or file parsing, then starts the web server with routes.
- **`src/lib.rs`**: Declares public modules (`db`, `models`, `parser`, `handlers`) for reuse across the project.
- **`src/db.rs`**: Manages SQLite database. Functions include `init_db` (creates tables), `insert_block`/`insert_tx` (stores data), and various `query_*` functions (retrieves data).
- **`src/models.rs`**: Defines structs for API responses (e.g., `BlockResponse`, `StatsResponse`) and internal data (e.g., `BlockSummary`). Uses Serde for JSON serialization.
- **`src/handlers.rs`**: Contains async functions for each API endpoint. Each handler locks the database, queries data, and returns JSON responses.
- **`src/parser.rs`**: Parses Bitcoin blocks from `.blk` files. Reads file streams, checks magic bytes, and deserializes blocks using the `bitcoin` crate.

## Installation

### From Crates.io (Recommended)

Once published:

```bash
cargo install regtest-block-explorer
```

### From Source

1. Clone the repository:
   ```bash
   git clone https://github.com/Macnelson9/Rust_Project.git
   cd Rust_Project
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. (Optional) Run tests:
   ```bash
   cargo test
   ```

## Usage

The tool has two main commands: `index` (to populate the database) and `serve` (to start the API).

### Prerequisites

- A running Bitcoin regtest node for RPC indexing (optional if using file mode).
- For file indexing: Access to Bitcoin regtest block files (e.g., `~/.bitcoin/regtest/blocks/blk00000.dat`).

### Commands

#### 1. Index Blocks

Fetches and stores block data in the database.

**Syntax**:

```bash
bitcoin-explore index [--from-file <PATH>]
```

**Options**:

- `--from-file <PATH>`: (Optional) Path to the directory containing `.blk` files (e.g., `/home/user/.bitcoin/regtest/blocks`). If omitted, uses RPC from a local regtest node.

**Examples**:

- RPC mode (requires running regtest node on port 18443):

  ```bash
  regtest-block-explorer index
  ```

  - Significance: Connects to `http://127.0.0.1:18443` via RPC, fetches all blocks, and indexes them. Useful for live data from a node.

- File mode:
  ```bash
  regtest-block-explorer index --from-file ~/.bitcoin/regtest/blocks
  ```
  - Significance: Parses `.blk` files directly. No node required; faster for existing data, but data must be available locally.

**Output**: Progress messages (e.g., "Indexed block at height X"). Creates/updates `blocks.db` in the current directory.

#### 2. Serve API

Starts the web server to query indexed data.

**Syntax**:

```bash
regtest-block-explorer serve [--port <PORT>]
```

**Options**:

- `--port <PORT>`: (Optional) Port to run the server on (default: 8080).

**Example**:

```bash
regtest-block-explorer serve --port 3000
```

- Significance: Launches an HTTP server. Access endpoints at `http://127.0.0.1:<PORT>`. Requires prior indexing; serves data from `blocks.db`.

**Output**: Lists available endpoints and starts listening. Use Ctrl+C to stop.

### Full Workflow Example

1. Start a regtest node (if using RPC):
   ```bash
   bitcoind -regtest -rpcuser=user -rpcpassword=pass -rpcport=18443 -datadir=$HOME/.bitcoin
   ```
2. Index blocks:
   ```bash
   regtest-block-explorer index
   ```
3. Serve the API:
   ```bash
   regtest-block-explorer serve
   ```
4. Query data:
   ```bash
   curl http://127.0.0.1:8080/stats
   ```

## API Endpoints

All endpoints return JSON. Run `regtest-block-explorer serve` to start the server.

- **`GET /block/{hash}`**: Get a block by its hash.

  - Example: `curl http://127.0.0.1:8080/block/00000000...`
  - Response: Full block details (height, transactions, etc.).

- **`GET /block/height/{height}`**: Get a block by height.

  - Example: `curl http://127.0.0.1:8080/block/height/0`
  - Response: Block data for the specified height.

- **`GET /tx/{txid}`**: Get a transaction by ID.

  - Example: `curl http://127.0.0.1:8080/tx/abcdef...`
  - Response: Transaction details (inputs, outputs, etc.).

- **`GET /blocks/latest?limit=10`**: Get the latest blocks.

  - Query params: `limit` (default: 10, max: 100).
  - Example: `curl "http://127.0.0.1:8080/blocks/latest?limit=5"`
  - Response: Array of recent blocks.

- **`GET /stats`**: Get blockchain statistics.

  - Example: `curl http://127.0.0.1:8080/stats`
  - Response: Total blocks, transactions, latest block info.

- **`GET /health`**: Health check.

  - Example: `curl http://127.0.0.1:8080/health`
  - Response: `{"status": "healthy", "service": "block-explorer-backend"}`

- **`GET /blocks?page=1&limit=20`**: Get all blocks with pagination.
  - Query params: `page` (default: 1), `limit` (default: 20, max: 100).
  - Example: `curl "http://127.0.0.1:8080/blocks?page=2&limit=10"`
  - Response: Paginated list with metadata (total pages, has_next, etc.).

## Contributing

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature-name`.
3. Make changes and run tests: `cargo test`.
4. Submit a pull request.

For issues or suggestions, open an issue on GitHub.

## License

This project is licensed under the MIT License or Apache-2.0 (see `LICENSE` file).

---

For beginners: This tool introduces Rust concepts like async programming, database interactions, and web APIs. Explore the code to learn more!
