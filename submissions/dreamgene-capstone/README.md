# Lightning Ticket Seller (ldk-node)

Rust CLI that runs a self-custodial Lightning node (ldk-node) on testnet. It creates BOLT11 invoices, shows a terminal QR, waits for payment, then returns the payment preimage as the "ticket" (with an optional QR file). Buyers use any Lightning wallet they already have.

## Commands

Examples assume you run from the repo root.

```bash
# Create an invoice and wait until it's paid, then emit the ticket
cargo run -- buy-ticket --event bitcoin-prague-2026 --price 100000 --save-qr invoice.png --ticket-qr ticket.png

# Create an invoice only (no waiting)
cargo run -- create-invoice --amount 25000 --memo "merch pack"

# Inspect known payments
cargo run -- list-payments

# Show your node id, configured listening addresses, and a funding address for channels
cargo run -- node-info

# (Demo helper) Connect to a peer and open a channel
cargo run -- --listen 0.0.0.0:9735 open-channel --peer <node_id_hex> --addr <host:port> --amount 200000 --push-msat 50000
```

Flags:
- `--data-dir` (default `./ldk-data`) stores the node seed, sqlite db, and channel data.
- `--esplora` (default testnet Blockstream endpoint) sets the chain data source.
- `--esplora-fallback` (default testnet mempool.space endpoint) used if the primary is unavailable/rate-limited.
- `--gossip` (default Rapid Gossip Sync testnet snapshot) accelerates routing table sync.
- `--expiry-secs` (default `3600`) invoice lifetime.
- `--listen` (comma-separated `ip:port`) to accept inbound peers/channels; pass when running commands, e.g. `cargo run -- --listen 0.0.0.0:9735 node-info`.
- `--save-qr` and `--ticket-qr` write PNGs; otherwise QR renders in the terminal.

## How it works

1. Node boots on testnet with ldk-node and syncs headers via Esplora plus Rapid Gossip Sync.
2. `receive_payment` produces a BOLT11 invoice string. The CLI prints it and renders a QR.
3. When the buyer pays from any wallet, the node sees `PaymentReceived`, claims funds, and on `PaymentClaimed` prints the preimage as the ticket (and QR if requested).

## Notes

- This targets **testnet** by default; swap URLs for mainnet if desired.
- Running the node requires network access for Esplora + gossip + peer connections.
- If the ldk-node API version differs, adjust the builder fields accordingly (network, storage dir, Esplora URL, Rapid Gossip Sync URL are the required knobs).
- If the primary Esplora (Blockstream by default) returns HTTP 429 or is down, the CLI retries with `--esplora-fallback`. Override with your own endpoint, or set it equal to `--esplora` to disable fallback.
