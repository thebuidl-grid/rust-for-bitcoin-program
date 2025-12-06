use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use crate::transaction::Transaction;
use std::collections::HashMap;

/// A block in the blockchain
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    /// Block version
    pub version: u32,
    /// Hash of previous block
    pub prev_hash: String,
    /// Merkle root of transactions
    pub merkle_root: String,
    /// Block timestamp
    pub timestamp: i64,
    /// Difficulty target (bits)
    pub bits: u32,
    /// Nonce for proof-of-work
    pub nonce: u64,
    /// Transactions in this block
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Create a new block
    pub fn new(
        prev_hash: String,
        transactions: Vec<Transaction>,
        bits: u32,
    ) -> Self {
        let merkle_root = Self::calculate_merkle_root(&transactions);
        Self {
            version: 1,
            prev_hash,
            merkle_root,
            timestamp: chrono::Utc::now().timestamp(),
            bits,
            nonce: 0,
            transactions,
        }
    }

    /// Calculate block hash
    pub fn hash(&self) -> String {
        // Create a header (simplified - real Bitcoin has specific header format)
        let header = format!(
            "{}{}{}{}{}{}",
            self.version,
            self.prev_hash,
            self.merkle_root,
            self.timestamp,
            self.bits,
            self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(header.as_bytes());
        let hash = hasher.finalize();
        
        // Double SHA256 (like Bitcoin)
        let mut hasher2 = Sha256::new();
        hasher2.update(&hash);
        let final_hash = hasher2.finalize();
        
        hex::encode(final_hash)
    }

    /// Calculate Merkle root (simplified - uses transaction IDs)
    fn calculate_merkle_root(transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return "0".repeat(64);
        }

        let mut hashes: Vec<String> = transactions.iter().map(|tx| tx.id()).collect();

        // Build Merkle tree
        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            
            for i in (0..hashes.len()).step_by(2) {
                let left = &hashes[i];
                let right = if i + 1 < hashes.len() {
                    &hashes[i + 1]
                } else {
                    left // Duplicate last element if odd number
                };

                let mut hasher = Sha256::new();
                hasher.update(left.as_bytes());
                hasher.update(right.as_bytes());
                let hash = hasher.finalize();
                next_level.push(hex::encode(hash));
            }

            hashes = next_level;
        }

        hashes[0].clone()
    }

    /// Get difficulty target from bits (simplified)
    /// Returns the number of leading zeros required
    pub fn required_leading_zeros(&self) -> usize {
        // bits represents the number of leading zero hex characters required
        // Each hex character is 4 bits, so bits/4 gives us hex characters
        (self.bits / 4) as usize
    }

    /// Check if block hash meets difficulty target
    pub fn meets_target(&self) -> bool {
        let hash = self.hash();
        let required_zeros = self.required_leading_zeros();
        
        // Count leading zeros in the hash
        let leading_zeros = hash.chars().take_while(|&c| c == '0').count();
        
        leading_zeros >= required_zeros
    }

    /// Mine the block (find valid nonce)
    pub fn mine(&mut self) {
        while !self.meets_target() {
            self.nonce += 1;
            
            // Prevent overflow
            if self.nonce == u64::MAX {
                self.timestamp = chrono::Utc::now().timestamp();
                self.nonce = 0;
            }
        }
    }

    /// Validate block structure
    pub fn is_valid_structure(&self) -> bool {
        // Must have at least one transaction (coinbase)
        if self.transactions.is_empty() {
            return false;
        }

        // First transaction must be coinbase
        if !self.transactions[0].is_coinbase() {
            return false;
        }

        // Merkle root must match calculated root
        let calculated_root = Self::calculate_merkle_root(&self.transactions);
        if self.merkle_root != calculated_root {
            return false;
        }

        true
    }

    /// Validate block against blockchain rules
    pub fn is_valid(
        &self,
        utxo_set: &HashMap<String, Vec<crate::transaction::TxOutput>>,
        expected_prev_hash: &str,
    ) -> Result<(), String> {
        // Check structure
        if !self.is_valid_structure() {
            return Err("Invalid block structure".to_string());
        }

        // Check previous hash matches
        if self.prev_hash != expected_prev_hash {
            return Err(format!(
                "Previous hash mismatch: expected {}, got {}",
                expected_prev_hash, self.prev_hash
            ));
        }

        // Validate all transactions
        for (i, tx) in self.transactions.iter().enumerate() {
            // Coinbase is always valid
            if i == 0 && tx.is_coinbase() {
                continue;
            }

            match tx.is_valid(utxo_set) {
                Ok(()) => {}
                Err(e) => return Err(format!("Invalid transaction {}: {}", i, e)),
            }
        }

        // Check proof-of-work
        if !self.meets_target() {
            return Err("Block does not meet difficulty target".to_string());
        }

        Ok(())
    }
}

