use rusqlite::{Connection, Result as SqlResult};
use crate::block::Block;
use crate::transaction::Transaction;
use serde_json;

/// Database for indexing blocks and transactions
pub struct BlockDatabase {
    conn: Connection,
}

impl BlockDatabase {
    /// Create or open database
    pub fn new(db_path: &str) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Initialize database schema
    fn init_schema(&self) -> SqlResult<()> {
        // Blocks table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS blocks (
                hash TEXT PRIMARY KEY,
                prev_hash TEXT,
                height INTEGER,
                timestamp INTEGER,
                merkle_root TEXT,
                nonce INTEGER,
                bits INTEGER,
                data TEXT
            )",
            [],
        )?;

        // Transactions table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS transactions (
                txid TEXT PRIMARY KEY,
                block_hash TEXT,
                block_height INTEGER,
                timestamp INTEGER,
                is_coinbase INTEGER,
                data TEXT,
                FOREIGN KEY(block_hash) REFERENCES blocks(hash)
            )",
            [],
        )?;

        // Create indexes
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_blocks_height ON blocks(height)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_blocks_prev_hash ON blocks(prev_hash)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tx_block_hash ON transactions(block_hash)",
            [],
        )?;

        Ok(())
    }

    /// Add a block to the database
    pub fn add_block(&self, block: &Block, height: usize) -> SqlResult<()> {
        let hash = block.hash();
        let data = serde_json::to_string(block)
            .map_err(|e| rusqlite::Error::InvalidColumnType(0, format!("JSON error: {}", e), 0))?;

        self.conn.execute(
            "INSERT OR REPLACE INTO blocks (hash, prev_hash, height, timestamp, merkle_root, nonce, bits, data)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                hash,
                block.prev_hash,
                height as i64,
                block.timestamp,
                block.merkle_root,
                block.nonce as i64,
                block.bits as i64,
                data
            ],
        )?;

        // Add all transactions
        for tx in &block.transactions {
            self.add_transaction(tx, &hash, height)?;
        }

        Ok(())
    }

    /// Add a transaction to the database
    pub fn add_transaction(
        &self,
        tx: &Transaction,
        block_hash: &str,
        block_height: usize,
    ) -> SqlResult<()> {
        let txid = tx.id();
        let data = serde_json::to_string(tx)
            .map_err(|e| rusqlite::Error::InvalidColumnType(0, format!("JSON error: {}", e), 0))?;

        self.conn.execute(
            "INSERT OR REPLACE INTO transactions (txid, block_hash, block_height, timestamp, is_coinbase, data)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                txid,
                block_hash,
                block_height as i64,
                tx.timestamp,
                if tx.is_coinbase() { 1 } else { 0 },
                data
            ],
        )?;

        Ok(())
    }

    /// Get block by hash
    pub fn get_block(&self, hash: &str) -> SqlResult<Option<Block>> {
        let mut stmt = self.conn.prepare(
            "SELECT data FROM blocks WHERE hash = ?1"
        )?;

        let mut rows = stmt.query_map([hash], |row| {
            let data: String = row.get(0)?;
            Ok(data)
        })?;

        if let Some(row) = rows.next() {
            let data = row?;
            let block: Block = serde_json::from_str(&data)
                .map_err(|e| rusqlite::Error::InvalidColumnType(0, format!("JSON error: {}", e), 0))?;
            Ok(Some(block))
        } else {
            Ok(None)
        }
    }

    /// Get transaction by ID
    pub fn get_transaction(&self, txid: &str) -> SqlResult<Option<Transaction>> {
        let mut stmt = self.conn.prepare(
            "SELECT data FROM transactions WHERE txid = ?1"
        )?;

        let mut rows = stmt.query_map([txid], |row| {
            let data: String = row.get(0)?;
            Ok(data)
        })?;

        if let Some(row) = rows.next() {
            let data = row?;
            let tx: Transaction = serde_json::from_str(&data)
                .map_err(|e| rusqlite::Error::InvalidColumnType(0, format!("JSON error: {}", e), 0))?;
            Ok(Some(tx))
        } else {
            Ok(None)
        }
    }

    /// Get block height
    pub fn get_height(&self) -> SqlResult<usize> {
        let mut stmt = self.conn.prepare("SELECT MAX(height) FROM blocks")?;
        let height: Option<i64> = stmt.query_row([], |row| row.get(0))?;
        Ok((height.unwrap_or(-1) + 1) as usize)
    }

    /// Get block height by hash
    pub fn get_block_height(&self, hash: &str) -> SqlResult<Option<usize>> {
        let mut stmt = self.conn.prepare("SELECT height FROM blocks WHERE hash = ?1")?;
        let height: Option<i64> = stmt.query_row([hash], |row| row.get(0))?;
        Ok(height.map(|h| h as usize))
    }

    /// Get transaction block info
    pub fn get_transaction_block_info(&self, txid: &str) -> SqlResult<Option<(String, usize)>> {
        let mut stmt = self.conn.prepare(
            "SELECT block_hash, block_height FROM transactions WHERE txid = ?1"
        )?;
        
        let mut rows = stmt.query_map([txid], |row| {
            let block_hash: String = row.get(0)?;
            let block_height: i64 = row.get(1)?;
            Ok((block_hash, block_height as usize))
        })?;

        if let Some(row) = rows.next() {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    /// Get all blocks (for syncing)
    pub fn get_all_blocks(&self) -> SqlResult<Vec<(String, Block)>> {
        let mut stmt = self.conn.prepare("SELECT hash, data FROM blocks ORDER BY height")?;
        let rows = stmt.query_map([], |row| {
            let hash: String = row.get(0)?;
            let data: String = row.get(1)?;
            Ok((hash, data))
        })?;

        let mut blocks = Vec::new();
        for row in rows {
            let (hash, data) = row?;
            let block: Block = serde_json::from_str(&data)
                .map_err(|e| rusqlite::Error::InvalidColumnType(0, format!("JSON error: {}", e), 0))?;
            blocks.push((hash, block));
        }

        Ok(blocks)
    }

    /// Clear all data
    pub fn clear(&self) -> SqlResult<()> {
        self.conn.execute("DELETE FROM transactions", [])?;
        self.conn.execute("DELETE FROM blocks", [])?;
        Ok(())
    }
}


