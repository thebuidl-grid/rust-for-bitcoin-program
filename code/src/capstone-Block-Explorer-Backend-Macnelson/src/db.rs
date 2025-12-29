use rusqlite::{Connection, Result};
use std::path::Path;
use crate::models::*;  

// Initialize DB and create tables
pub fn init_db(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS blocks (
            hash TEXT PRIMARY KEY,
            height INTEGER,
            version INTEGER,
            prev_block TEXT,
            merkle_root TEXT,
            timestamp INTEGER,
            bits INTEGER,
            nonce INTEGER,
            size INTEGER,
            header BLOB,
            raw_data BLOB
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            txid TEXT PRIMARY KEY,
            block_hash TEXT,
            inputs TEXT,
            outputs TEXT,
            raw_data BLOB,
            FOREIGN KEY (block_hash) REFERENCES blocks(hash)
        )",
        [],
    )?;
    Ok(conn)
}



// Function to insert a block
pub fn insert_block(conn: &Connection, block: &bitcoin::Block, height: u32) -> Result<()> {
    let hash = block.block_hash().to_string();
    let header = &block.header;
    let header_blob = bitcoin::consensus::encode::serialize(header);
    let raw_data = bitcoin::consensus::encode::serialize(block);

    conn.execute(
        "INSERT OR REPLACE INTO blocks (hash, height, version, prev_block, merkle_root, timestamp, bits, nonce, size, header, raw_data) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        rusqlite::params![
            &hash,
            &height,
            &header.version.to_consensus(),
            &header.prev_blockhash.to_string(),
            &header.merkle_root.to_string(),
            &header.time,
            &header.bits.to_consensus(),
            &header.nonce,
            &raw_data.len(),
            &header_blob,
            &raw_data
        ],
    )?;

    for tx in &block.txdata {
        insert_tx(conn, tx, &hash)?;
    }
    Ok(())
}

// Function to insert a transaction
// FIXED: Convert TxIn/TxOut to serializable versions
pub fn insert_tx(conn: &Connection, tx: &bitcoin::Transaction, block_hash: &str) -> Result<()> {
    let txid = tx.compute_txid().to_string();
    
    // Convert inputs to simplified version
    let inputs: Vec<TxInSimplified> = tx.input.iter().map(|input| {
        TxInSimplified {
            prev_txid: input.previous_output.txid.to_string(),
            vout: input.previous_output.vout,
            script_sig: hex::encode(&input.script_sig.as_bytes()),
            sequence: input.sequence.0,
            witness: input.witness.iter()
                .map(|w| hex::encode(w))
                .collect(),
        }
    }).collect();
    
    // Convert outputs to simplified version
    let outputs: Vec<TxOutSimplified> = tx.output.iter().map(|output| {
        TxOutSimplified {
            value: output.value.to_sat(),
            script_pubkey: hex::encode(&output.script_pubkey.as_bytes()),
        }
    }).collect();
    
    let inputs_json = serde_json::to_string(&inputs).unwrap();
    let outputs_json = serde_json::to_string(&outputs).unwrap();
    let raw_data = bitcoin::consensus::encode::serialize(tx);

    conn.execute(
        "INSERT OR REPLACE INTO transactions (txid, block_hash, inputs, outputs, raw_data) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![&txid, block_hash, &inputs_json, &outputs_json, &raw_data],
    )?;
    Ok(())
}

pub fn query_block(conn: &Connection, hash: &str) -> Result<Option<BlockResponse>> {
    let mut stmt = conn.prepare(
        "SELECT hash, height, version, prev_block, merkle_root, timestamp, bits, nonce, size FROM blocks WHERE hash = ?1"
    )?;
    
    let mut block_iter = stmt.query_map([hash], |row| {
        let mut block = BlockResponse {
            hash: row.get(0)?,
            height: row.get(1)?,
            version: row.get(2)?,
            prev_block: row.get(3)?,
            merkle_root: row.get(4)?,
            timestamp: row.get(5)?,
            bits: row.get(6)?,
            nonce: row.get(7)?,
            tx_count: 0,  // Will update below
            size: row.get(8)?,
        };
        
        // Count transactions for this block
        let tx_count: usize = conn.query_row(
            "SELECT COUNT(*) FROM transactions WHERE block_hash = ?1",
            [hash],
            |r| r.get(0)
        )?;
        block.tx_count = tx_count;
        
        Ok(block)
    })?;
    
    if let Some(block) = block_iter.next() {
        Ok(Some(block?))
    } else {
        Ok(None)
    }
}

pub fn query_block_by_height(conn: &Connection, height: u32) -> Result<Option<BlockResponse>> {
    let hash: Option<String> = conn.query_row(
        "SELECT hash FROM blocks WHERE height = ?1",
        [height],
        |row| row.get(0)
    ).ok();
    
    if let Some(hash) = hash {
        query_block(conn, &hash)
    } else {
        Ok(None)
    }
}

pub fn query_tx(conn: &Connection, txid: &str) -> Result<Option<TxResponse>> {
    let mut stmt = conn.prepare(
        "SELECT txid, block_hash, inputs, outputs FROM transactions WHERE txid = ?1"
    )?;
    
    let mut tx_iter = stmt.query_map([txid], |row| {
        let txid: String = row.get(0)?;
        let block_hash: Option<String> = row.get(1)?;
        let inputs: String = row.get(2)?;
        let outputs: String = row.get(3)?;
        
        let inputs: Vec<TxInSimplified> = serde_json::from_str(&inputs).unwrap_or_default();
        let outputs: Vec<TxOutSimplified> = serde_json::from_str(&outputs).unwrap_or_default();
        
        // Get block height if available
        let block_height = if let Some(ref hash) = block_hash {
            conn.query_row(
                "SELECT height FROM blocks WHERE hash = ?1",
                [hash],
                |r| r.get(0)
            ).ok()
        } else {
            None
        };
        
        Ok(TxResponse {
            txid,
            version: 1,  // Placeholder; could store in DB if needed
            lock_time: 0,  // Placeholder
            block_hash,
            block_height,
            confirmations: None,  // Placeholder
            inputs,
            outputs,
            size: 0,  // Placeholder; calculate if needed
            vsize: 0,  // Placeholder
            weight: 0,  // Placeholder
        })
    })?;
    
    if let Some(tx) = tx_iter.next() {
        Ok(Some(tx?))
    } else {
        Ok(None)
    }
}

pub fn query_latest_blocks(conn: &Connection, limit: usize) -> Result<Vec<BlockSummary>> {
    let mut stmt = conn.prepare(
        "SELECT hash, height, timestamp FROM blocks ORDER BY height DESC LIMIT ?1"
    )?;
    
    let rows = stmt.query_map([limit], |row| {
        let hash: String = row.get(0)?;
        let height: u32 = row.get(1)?;
        let timestamp: u32 = row.get(2)?;
        
        let tx_count: usize = conn.query_row(
            "SELECT COUNT(*) FROM transactions WHERE block_hash = ?1",
            [&hash],
            |r| r.get(0)
        )?;
        
        Ok(BlockSummary {
            hash,
            height,
            timestamp,
            tx_count,
        })
    })?;
    
    let mut blocks = Vec::new();
    for row in rows {
        blocks.push(row?);
    }
    Ok(blocks)
}

pub fn query_block_count(conn: &Connection) -> Result<u32> {
    conn.query_row("SELECT COUNT(*) FROM blocks", [], |row| row.get(0))
}

pub fn query_transaction_count(conn: &Connection) -> Result<u64> {
    conn.query_row("SELECT COUNT(*) FROM transactions", [], |row| row.get(0))
}

pub fn query_latest_block(conn: &Connection) -> Result<Option<(u32, String)>> {
    match conn.query_row(
        "SELECT height, hash FROM blocks ORDER BY height DESC LIMIT 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?))
    ) {
        Ok(data) => Ok(Some(data)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn query_all_blocks(conn: &Connection, limit: usize, offset: usize) -> Result<Vec<BlockSummary>> {
    let mut stmt = conn.prepare(
        "SELECT hash, height, timestamp FROM blocks ORDER BY height DESC LIMIT ?1 OFFSET ?2"
    )?;
    
    let rows = stmt.query_map([limit, offset], |row| {
        let hash: String = row.get(0)?;
        let height: u32 = row.get(1)?;
        let timestamp: u32 = row.get(2)?;
        
        let tx_count: usize = conn.query_row(
            "SELECT COUNT(*) FROM transactions WHERE block_hash = ?1",
            [&hash],
            |r| r.get(0)
        )?;
        
        Ok(BlockSummary {
            hash,
            height,
            timestamp,
            tx_count,
        })
    })?;
    
    let mut blocks = Vec::new();
    for row in rows {
        blocks.push(row?);
    }
    Ok(blocks)
}