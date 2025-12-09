use clap::{Parser, Subcommand};
// use rusqlite::Connection;  // REMOVE THIS LINE
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use actix_web::{web, App, HttpServer, middleware};
use env_logger;
use reqwest;
use serde_json::json;
use bitcoin::consensus;

mod models;
mod db;
mod parser;
mod handlers;

use db::*;
use handlers::*;

#[derive(Parser)]
#[command(name = "bitcoin-explore")]
#[command(about = "A CLI tool for Bitcoin regtest block indexing and exploration")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Index blocks from a running regtest node via RPC or from local .blk files
    Index {
        /// Path to blocks directory for file-based indexing (optional; if not provided, uses RPC)
        #[arg(long)]
        from_file: Option<String>,
    },
    /// Start the web server for block exploration API
    Serve {
        /// Port to run the server on
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    let db_path = Path::new("blocks.db");
    let conn = Arc::new(Mutex::new(init_db(db_path)?));

    match cli.command {
        Commands::Index { from_file } => {
            if let Some(path) = from_file {
                println!("Indexing from files in: {}", path);
                parser::index_blocks(&conn.lock().unwrap(), Path::new(&path)).await?;
                println!("File-based indexing complete!");
            } else {
                println!("Block Explorer Indexer");
                println!("Fetching blocks from regtest node at http://127.0.0.1:18443");

                let client = reqwest::Client::new();

                // Get block count
                let response = client
                    .post("http://127.0.0.1:18443")
                    .basic_auth("user", Some("pass"))
                    .json(&json!({"jsonrpc": "1.0", "id": "1", "method": "getblockcount", "params": []}))
                    .send()
                    .await?;
                let result: serde_json::Value = response.json().await?;
                let count = result["result"].as_u64().unwrap_or(0) as u32;

                println!("Starting block indexing... Total blocks: {}", count);

                for height in 0..count {
                    // Get block hash
                    let response = client
                        .post("http://127.0.0.1:18443")
                        .basic_auth("user", Some("pass"))
                        .json(&json!({"jsonrpc": "1.0", "id": "1", "method": "getblockhash", "params": [height]}))
                        .send()
                        .await?;
                    let result: serde_json::Value = response.json().await?;
                    let hash = result["result"].as_str().unwrap();

                    // Get block hex
                    let response = client
                        .post("http://127.0.0.1:18443")
                        .basic_auth("user", Some("pass"))
                        .json(&json!({"jsonrpc": "1.0", "id": "1", "method": "getblock", "params": [hash, 0]}))
                        .send()
                        .await?;
                    let result: serde_json::Value = response.json().await?;
                    let hex = result["result"].as_str().unwrap();

                    let block_bytes = hex::decode(hex).unwrap();
                    let block: bitcoin::Block = consensus::deserialize(&block_bytes).unwrap();

                    insert_block(&conn.lock().unwrap(), &block, height)?;
                    println!("Indexed block at height {}: {}", height, hash);
                }

                let block_count: u32 = conn.lock().unwrap().query_row(
                    "SELECT COUNT(*) FROM blocks", [], |row| row.get(0)
                )?;
                let tx_count: u64 = conn.lock().unwrap().query_row(
                    "SELECT COUNT(*) FROM transactions", [], |row| row.get(0)
                )?;

                println!("Indexing complete!");
                println!("Blocks: {}", block_count);
                println!("Transactions: {}", tx_count);
            }
        }
        Commands::Serve { port } => {
            println!("Starting web server on http://127.0.0.1:{}", port);
            println!("Available endpoints:");
            println!("  GET /block/{{hash}} - Get block by hash");
            println!("  GET /block/height/{{height}} - Get block by height");
            println!("  GET /tx/{{txid}} - Get transaction by ID");
            println!("  GET /blocks/latest?limit=10 - Get latest blocks");
            println!("  GET /stats - Get blockchain statistics");
            println!("  GET /health - Health check");
            println!("  GET /blocks?page=1&limit=20 - Get all blocks with pagination");
            let conn_clone = Arc::clone(&conn);
            HttpServer::new(move || {
                App::new()
                    .wrap(middleware::Logger::default())
                    .app_data(web::Data::new(conn_clone.clone()))
                    .route("/block/{hash}", web::get().to(get_block))
                    .route("/block/height/{height}", web::get().to(get_block_by_height))
                    .route("/tx/{txid}", web::get().to(get_tx))
                    .route("/blocks/latest", web::get().to(get_latest_blocks))
                    .route("/stats", web::get().to(get_stats))
                    .route("/health", web::get().to(health_check))
                    .route("/blocks", web::get().to(get_all_blocks))
            })
            .bind(("127.0.0.1", port))?
            .run()
            .await?;
        }
    }

    Ok(())
}