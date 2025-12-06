use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use crate::block::Block;
use crate::transaction::Transaction;
use crate::database::BlockDatabase;
use std::sync::Arc;
use tokio::sync::RwLock;

/// API response types
#[derive(Serialize)]
pub struct BlockResponse {
    pub hash: String,
    pub prev_hash: String,
    pub height: usize,
    pub timestamp: i64,
    pub merkle_root: String,
    pub nonce: u64,
    pub bits: u32,
    pub transaction_count: usize,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub txid: String,
    pub block_hash: Option<String>,
    pub block_height: Option<usize>,
    pub timestamp: i64,
    pub is_coinbase: bool,
    pub input_count: usize,
    pub output_count: usize,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// API state
pub struct ApiState {
    pub db: Arc<RwLock<BlockDatabase>>,
}

/// Create the API router
pub fn create_router(db: BlockDatabase) -> Router {
    let state = ApiState {
        db: Arc::new(RwLock::new(db)),
    };

    Router::new()
        .route("/block/:hash", get(get_block))
        .route("/tx/:txid", get(get_transaction))
        .route("/health", get(health_check))
        .with_state(state)
}

/// Health check endpoint
async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "status": "ok",
        "service": "bitcoin-block-explorer"
    })))
}

/// Get block by hash
async fn get_block(
    Path(hash): Path<String>,
    axum::extract::State(state): axum::extract::State<ApiState>,
) -> Result<Json<BlockResponse>, (StatusCode, Json<ErrorResponse>)> {
    let db = state.db.read().await;
    
    match db.get_block(&hash) {
        Ok(Some(block)) => {
            let height = db.get_block_height(&hash)
                .ok()
                .flatten()
                .unwrap_or(0);
            
            let response = BlockResponse {
                hash: block.hash(),
                prev_hash: block.prev_hash,
                height,
                timestamp: block.timestamp,
                merkle_root: block.merkle_root,
                nonce: block.nonce,
                bits: block.bits,
                transaction_count: block.transactions.len(),
            };
            Ok(Json(response))
        }
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Block not found: {}", hash),
            }),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Database error: {}", e),
            }),
        )),
    }
}

/// Get transaction by ID
async fn get_transaction(
    Path(txid): Path<String>,
    axum::extract::State(state): axum::extract::State<ApiState>,
) -> Result<Json<TransactionResponse>, (StatusCode, Json<ErrorResponse>)> {
    let db = state.db.read().await;
    
    match db.get_transaction(&txid) {
        Ok(Some(tx)) => {
            // Get block info for this transaction
            let block_info = db.get_transaction_block_info(&txid)
                .ok()
                .flatten();
            
            let (block_hash, block_height) = block_info.unwrap_or((String::new(), 0));
            
            let response = TransactionResponse {
                txid: tx.id(),
                block_hash: if block_hash.is_empty() { None } else { Some(block_hash) },
                block_height: if block_height == 0 { None } else { Some(block_height) },
                timestamp: tx.timestamp,
                is_coinbase: tx.is_coinbase(),
                input_count: tx.inputs.len(),
                output_count: tx.outputs.len(),
            };
            Ok(Json(response))
        }
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Transaction not found: {}", txid),
            }),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Database error: {}", e),
            }),
        )),
    }
}

/// Start the API server
pub async fn start_server(db: BlockDatabase, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_router(db);
    
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("API server listening on http://0.0.0.0:{}", port);
    println!("Endpoints:");
    println!("  GET /health");
    println!("  GET /block/:hash");
    println!("  GET /tx/:txid");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}


