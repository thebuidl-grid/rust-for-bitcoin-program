use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use hex::encode as hex_encode;
use image::Luma;
use ldk_node::bitcoin::secp256k1::PublicKey;
use ldk_node::io::sqlite_store::SqliteStore;
use ldk_node::lightning::ln::msgs::SocketAddress;
use ldk_node::{Builder, Event, Network, Node};
use qrcode::{QrCode, render::unicode};

#[derive(Parser)]
#[command(
    name = "lighting-node",
    about = "Minimal Lightning ticket seller built on ldk-node"
)]
struct Cli {
    /// Directory where ldk-node keeps its state (seed, channels, db)
    #[arg(long, default_value = "./ldk-data")]
    data_dir: PathBuf,
    /// Esplora HTTP endpoint for chain sync (testnet by default)
    #[arg(long, default_value = "https://blockstream.info/testnet/api")]
    esplora: String,
    /// Optional fallback Esplora endpoint if the primary is rate-limited/unavailable
    #[arg(long, default_value = "https://mempool.space/testnet/api")]
    esplora_fallback: String,
    /// Rapid Gossip Sync snapshot URL
    #[arg(
        long,
        default_value = "https://rapidsync.lightningdevkit.org/testnet/snapshot"
    )]
    gossip: String,
    /// How many seconds an invoice stays payable
    #[arg(long, default_value = "3600")]
    expiry_secs: u32,
    /// Optional listening addresses to accept inbound peers/channels (comma separated)
    #[arg(long, value_delimiter = ',', value_name = "ADDR:PORT")]
    listen: Vec<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create an invoice for a ticket and wait until it is paid to emit the ticket preimage
    BuyTicket {
        /// Event slug that will be echoed back in the ticket
        #[arg(long)]
        event: String,
        /// Price in sats
        #[arg(long)]
        price: u64,
        /// Save the invoice QR as a PNG (optional)
        #[arg(long)]
        save_qr: Option<PathBuf>,
        /// Save the ticket (payment preimage) QR after payment
        #[arg(long)]
        ticket_qr: Option<PathBuf>,
    },
    /// Just create an invoice (no waiting)
    CreateInvoice {
        /// Amount in sats
        #[arg(long)]
        amount: u64,
        /// Human readable description in the invoice
        #[arg(long)]
        memo: Option<String>,
        /// Save the invoice QR as a PNG (optional)
        #[arg(long)]
        save_qr: Option<PathBuf>,
    },
    /// Print known payments
    ListPayments,
    /// Show node id, listening addresses, and a new on-chain funding address
    NodeInfo,
    /// Connect to a peer and open a channel (for demos)
    OpenChannel {
        /// Peer node id (hex)
        #[arg(long, value_name = "HEX")]
        peer: String,
        /// Peer address host:port (or onion:port)
        #[arg(long, value_name = "HOST:PORT")]
        addr: String,
        /// Channel size in sats
        #[arg(long)]
        amount: u64,
        /// Push this many millisats to peer on open (gives you inbound)
        #[arg(long)]
        push_msat: Option<u64>,
        /// Announce the channel to the network
        #[arg(long, default_value_t = false)]
        announce: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let node = start_node(&cli)?;

    match &cli.command {
        Commands::BuyTicket {
            event,
            price,
            save_qr,
            ticket_qr,
        } => {
            let _invoice = create_invoice(
                &node,
                *price,
                Some(event.as_str()),
                cli.expiry_secs,
                save_qr.clone(),
            )?;

            println!();
            println!("Waiting for payment...");
            issue_ticket_on_payment(&node, event, ticket_qr.clone())?;
            println!("Invoice settled, shutting down node…");
        }
        Commands::CreateInvoice {
            amount,
            memo,
            save_qr,
        } => {
            create_invoice(
                &node,
                *amount,
                memo.as_deref(),
                cli.expiry_secs,
                save_qr.clone(),
            )?;
        }
        Commands::ListPayments => {
            list_payments(&node)?;
        }
        Commands::NodeInfo => {
            print_node_info(&node)?;
        }
        Commands::OpenChannel {
            peer,
            addr,
            amount,
            push_msat,
            announce,
        } => {
            open_channel(&node, peer, addr, *amount, *push_msat, *announce)?;
        }
    }

    node.stop().ok();
    Ok(())
}

type NodeHandle = Node<SqliteStore>;

fn start_node(cli: &Cli) -> Result<NodeHandle> {
    let mut esploras = vec![cli.esplora.clone()];
    if cli.esplora_fallback != cli.esplora {
        esploras.push(cli.esplora_fallback.clone());
    }

    let mut last_err: Option<anyhow::Error> = None;
    for (idx, esplora) in esploras.iter().enumerate() {
        let label = if idx == 0 { "primary" } else { "fallback" };
        match build_and_start_node(cli, esplora) {
            Ok(node) => {
                if idx > 0 {
                    println!("Started node with fallback Esplora: {esplora}");
                }
                return Ok(node);
            }
            Err(err) => {
                if idx + 1 < esploras.len() {
                    eprintln!(
                        "Starting node with {label} Esplora {esplora} failed: {err:?}. Trying fallback…"
                    );
                }
                last_err = Some(err);
            }
        }
    }

    Err(last_err.unwrap_or_else(|| anyhow!("failed to start ldk-node with any Esplora endpoint")))
}

fn build_and_start_node(cli: &Cli, esplora: &str) -> Result<NodeHandle> {
    let mut builder = Builder::new();
    builder.set_network(Network::Testnet);
    builder.set_storage_dir_path(cli.data_dir.to_string_lossy().into_owned());
    builder.set_esplora_server(esplora.to_owned());
    builder.set_gossip_source_rgs(cli.gossip.clone());
    if !cli.listen.is_empty() {
        let listening_addresses = cli
            .listen
            .iter()
            .map(|addr| {
                SocketAddress::from_str(addr).map_err(|_| anyhow!("invalid listen address: {addr}"))
            })
            .collect::<Result<Vec<_>>>()?;
        builder
            .set_listening_addresses(listening_addresses)
            .context("invalid listening address")?;
    }

    let node = builder.build().context("failed to build ldk-node")?;
    node.start()
        .with_context(|| format!("failed to start ldk-node using Esplora {esplora}"))?;
    Ok(node)
}

fn print_node_info(node: &NodeHandle) -> Result<()> {
    println!("Node ID: {}", node.node_id());
    match node.listening_addresses() {
        Some(addrs) if !addrs.is_empty() => {
            println!("Listening addresses:");
            for addr in addrs {
                println!("- {addr}");
            }
        }
        _ => {
            println!(
                "Listening addresses: none configured. Use --listen <ip:port> to accept inbound peers/channels."
            );
        }
    }
    let funding = node
        .new_onchain_address()
        .context("failed to derive on-chain address")?;
    println!("New on-chain funding address: {funding}");
    Ok(())
}

fn open_channel(
    node: &NodeHandle,
    peer_hex: &str,
    addr: &str,
    amount_sats: u64,
    push_msat: Option<u64>,
    announce: bool,
) -> Result<()> {
    let peer_id = PublicKey::from_str(peer_hex).context("invalid peer node id hex")?;
    let socket_addr =
        SocketAddress::from_str(addr).map_err(|_| anyhow!("invalid peer address: {addr}"))?;
    node.connect_open_channel(peer_id, socket_addr, amount_sats, push_msat, None, announce)
        .context("failed to open channel")?;
    println!(
        "Channel initiation sent to {peer_hex} at {addr} for {amount_sats} sats (push_msat: {}).",
        push_msat
            .map(|v| v.to_string())
            .unwrap_or_else(|| "0".to_string())
    );
    println!("Wait for on-chain confirmations, then the channel will become active.");
    Ok(())
}

fn create_invoice(
    node: &NodeHandle,
    amount_sats: u64,
    memo: Option<&str>,
    expiry_secs: u32,
    save_qr: Option<PathBuf>,
) -> Result<String> {
    let description = memo.unwrap_or("ticket");
    let invoice = node
        .receive_payment(amount_sats * 1000, description, expiry_secs)
        .context("failed to create invoice")?;

    let encoded = invoice.to_string();
    println!(
        "Invoice created! Pay exactly {} sats for 1 ticket.",
        amount_sats
    );
    println!();
    println!("{encoded}");
    render_qr_to_terminal(&encoded)?;
    if let Some(path) = save_qr {
        save_qr_png(&encoded, &path)?;
        println!("Saved invoice QR to {}", path.display());
    }
    println!("Or pay instantly in browser → https://pay.sendsats.to/{encoded}");
    Ok(encoded)
}

fn issue_ticket_on_payment(
    node: &NodeHandle,
    event: &str,
    ticket_qr: Option<PathBuf>,
) -> Result<()> {
    loop {
        let event_data = node.wait_next_event();
        match event_data {
            Event::PaymentReceived {
                payment_hash,
                amount_msat,
            } => {
                println!("Payment received! {} sats", amount_msat / 1000);
                if let Some(preimage_hex) = lookup_preimage_hex(node, &payment_hash) {
                    println!("Ticket issued for {event}!");
                    println!("Payment settled: {} sats", amount_msat / 1000);
                    println!();
                    println!("YOUR TICKET (preimage, show at the door):");
                    println!("{preimage_hex}");
                    render_qr_to_terminal(&preimage_hex)?;
                    if let Some(path) = ticket_qr {
                        save_qr_png(&preimage_hex, &path)?;
                        println!("Saved ticket QR to {}", path.display());
                    }
                } else {
                    println!(
                        "Payment recorded but preimage not yet available. Check list-payments later."
                    );
                }
                node.event_handled();
                break;
            }
            other => {
                println!("Ignoring node event: {:?}", other);
                node.event_handled();
            }
        }
    }
    Ok(())
}

fn list_payments(node: &NodeHandle) -> Result<()> {
    let payments = node.list_payments();
    if payments.is_empty() {
        println!("No payments yet.");
    } else {
        println!("Known payments:");
        for payment in payments {
            println!("{payment:?}");
        }
    }
    Ok(())
}

fn render_qr_to_terminal(data: &str) -> Result<()> {
    let code = QrCode::new(data.as_bytes()).context("failed to build QR")?;
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Dark)
        .light_color(unicode::Dense1x2::Light)
        .build();
    println!("{image}");
    Ok(())
}

fn save_qr_png(data: &str, path: &PathBuf) -> Result<()> {
    let code = QrCode::new(data.as_bytes()).context("failed to build QR")?;
    let image = code.render::<Luma<u8>>().min_dimensions(256, 256).build();
    image
        .save(path)
        .with_context(|| format!("unable to save QR to {}", path.display()))?;
    Ok(())
}

fn lookup_preimage_hex(
    node: &NodeHandle,
    payment_hash: &ldk_node::lightning::ln::PaymentHash,
) -> Option<String> {
    node.list_payments()
        .into_iter()
        .find(|p| &p.hash == payment_hash)
        .and_then(|p| p.preimage)
        .map(|pre| hex_encode(pre.0))
}
