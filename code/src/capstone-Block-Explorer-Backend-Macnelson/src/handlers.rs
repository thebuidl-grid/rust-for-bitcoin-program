use actix_web::{web, HttpResponse, Responder};
use rusqlite::Connection;
use serde_json::json;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use crate::db::*;
use crate::models::*;

pub async fn get_block(
    db: web::Data<Arc<Mutex<Connection>>>,
    hash: web::Path<String>,
) -> impl Responder {
    let hash = hash.into_inner();
    let conn = db.lock().unwrap();
    
    match query_block(&conn, &hash) {
        Ok(Some(block)) => HttpResponse::Ok().json(block),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": "Block not found",
            "hash": hash
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": "Database error",
            "message": e.to_string()
        })),
    }
}

pub async fn get_block_by_height(
    db: web::Data<Arc<Mutex<Connection>>>,
    height: web::Path<u32>,
) -> impl Responder {
    let height = height.into_inner();
    let conn = db.lock().unwrap();
    
    match query_block_by_height(&conn, height) {
        Ok(Some(block)) => HttpResponse::Ok().json(block),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": "Block not found",
            "height": height
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": "Database error",
            "message": e.to_string()
        })),
    }
}

pub async fn get_tx(
    db: web::Data<Arc<Mutex<Connection>>>,
    txid: web::Path<String>,
) -> impl Responder {
    let txid = txid.into_inner();
    let conn = db.lock().unwrap();
    
    match query_tx(&conn, &txid) {
        Ok(Some(tx)) => HttpResponse::Ok().json(tx),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": "Transaction not found",
            "txid": txid
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": "Database error",
            "message": e.to_string()
        })),
    }
}

pub async fn get_latest_blocks(
    db: web::Data<Arc<Mutex<Connection>>>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let limit: usize = query.get("limit")
        .and_then(|l| l.parse().ok())
        .unwrap_or(10);
    let conn = db.lock().unwrap();
    
    match query_latest_blocks(&conn, limit) {
        Ok(blocks) => {
            let total_count = blocks.len() as u32;  // Calculate BEFORE moving
            HttpResponse::Ok().json(LatestBlocksResponse {
                blocks,  // Now move
                total_count,
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": "Database error",
            "message": e.to_string()
        })),
    }
}
// GET /stats - Get blockchain statistics
pub async fn get_stats(
    db: web::Data<Arc<Mutex<Connection>>>,
) -> impl Responder {
    let conn = db.lock().unwrap();
    
    let total_blocks = crate::db::query_block_count(&conn).unwrap_or(0);
    let total_txs = crate::db::query_transaction_count(&conn).unwrap_or(0);
    
    let latest = crate::db::query_latest_block(&conn);
    
    match latest {
        Ok(Some((height, hash))) => {
            HttpResponse::Ok().json(StatsResponse {
                total_blocks,
                total_transactions: total_txs,
                latest_block_height: height,
                latest_block_hash: hash,
            })
        }
        _ => HttpResponse::Ok().json(serde_json::json!({
            "total_blocks": total_blocks,
            "total_transactions": total_txs,
            "message": "No blocks indexed yet"
        })),
    }
}

// GET /health - Health check endpoint
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "block-explorer-backend"
    }))
}

// GET /blocks?page=1&limit=20 - Get all blocks with pagination
pub async fn get_all_blocks(
    db: web::Data<Arc<Mutex<Connection>>>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let page: usize = query.get("page")
        .and_then(|p| p.parse().ok())
        .unwrap_or(1)
        .max(1); // Minimum page 1
    
    let limit: usize = query.get("limit")
        .and_then(|l| l.parse().ok())
        .unwrap_or(20)
        .min(100); // Max 100 blocks per page
    
    let offset = (page - 1) * limit;
    
    let conn = db.lock().unwrap();
    
    match crate::db::query_all_blocks(&conn, limit, offset) {
        Ok(blocks) => {
            let total = crate::db::query_block_count(&conn).unwrap_or(0);
            let total_pages = (total as f64 / limit as f64).ceil() as usize;
            
            HttpResponse::Ok().json(serde_json::json!({
                "blocks": blocks,
                "pagination": {
                    "current_page": page,
                    "per_page": limit,
                    "total_blocks": total,
                    "total_pages": total_pages,
                    "has_next": page < total_pages,
                    "has_prev": page > 1
                }
            }))
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Database error",
            "message": e.to_string()
        })),
    }
}
