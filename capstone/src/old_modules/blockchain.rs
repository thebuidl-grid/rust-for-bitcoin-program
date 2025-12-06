use crate::block::Block;
use crate::transaction::{Transaction, TxOutput};
use std::collections::HashMap;

/// Represents the entire blockchain
#[derive(Debug, Clone)]
pub struct Blockchain {
    /// Chain of blocks
    pub blocks: Vec<Block>,
    /// UTXO set: maps transaction ID to its outputs
    pub utxo_set: HashMap<String, Vec<TxOutput>>,
    /// Current difficulty (bits)
    pub difficulty: u32,
}

impl Blockchain {
    /// Create a new blockchain with genesis block
    pub fn new() -> Self {
        let mut blockchain = Self {
            blocks: Vec::new(),
            utxo_set: HashMap::new(),
            difficulty: 4, // Start with 1 leading zero (4 bits / 4 = 1 hex char)
        };

        // Create genesis block
        let genesis = blockchain.create_genesis_block();
        blockchain.add_block(genesis).unwrap();
        
        blockchain
    }

    /// Create the genesis block
    fn create_genesis_block(&self) -> Block {
        // Genesis block has no previous hash
        let prev_hash = "0".repeat(64);
        
        // Create coinbase transaction (mining reward)
        let coinbase = Transaction::new(
            vec![], // No inputs
            vec![crate::transaction::TxOutput {
                amount: 50_0000_0000, // 50 BTC in satoshis
                address: "genesis".to_string(),
            }],
        );

        let mut block = Block::new(prev_hash, vec![coinbase], self.difficulty);
        block.mine();
        block
    }

    /// Get the latest block hash
    pub fn latest_hash(&self) -> String {
        if self.blocks.is_empty() {
            "0".repeat(64)
        } else {
            self.blocks.last().unwrap().hash()
        }
    }

    /// Get the latest block
    pub fn latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    /// Add a block to the chain
    pub fn add_block(&mut self, block: Block) -> Result<(), String> {
        // Validate the block
        let expected_prev_hash = self.latest_hash();
        block.is_valid(&self.utxo_set, &expected_prev_hash)?;

        // Update UTXO set
        self.update_utxo_set(&block);

        // Add block to chain
        self.blocks.push(block);

        Ok(())
    }

    /// Update UTXO set when a block is added
    fn update_utxo_set(&mut self, block: &Block) {
        // Remove spent UTXOs (transaction inputs)
        for tx in &block.transactions {
            if !tx.is_coinbase() {
                for input in &tx.inputs {
                    // Remove the spent output
                    if let Some(outputs) = self.utxo_set.get_mut(&input.prev_tx_id) {
                        if (input.output_index as usize) < outputs.len() {
                            outputs.remove(input.output_index as usize);
                            
                            // Remove transaction from UTXO set if all outputs are spent
                            if outputs.is_empty() {
                                self.utxo_set.remove(&input.prev_tx_id);
                            }
                        }
                    }
                }
            }
        }

        // Add new UTXOs (transaction outputs)
        for tx in &block.transactions {
            let tx_id = tx.id();
            let outputs: Vec<TxOutput> = tx.outputs.clone();
            self.utxo_set.insert(tx_id, outputs);
        }
    }

    /// Validate the entire chain
    pub fn is_valid(&self) -> Result<(), String> {
        if self.blocks.is_empty() {
            return Err("Blockchain is empty".to_string());
        }

        // Rebuild UTXO set to validate
        let mut utxo_set = HashMap::new();
        let mut prev_hash = "0".repeat(64);

        for (i, block) in self.blocks.iter().enumerate() {
            // Validate block structure
            if !block.is_valid_structure() {
                return Err(format!("Invalid block structure at height {}", i));
            }

            // Validate previous hash
            if block.prev_hash != prev_hash {
                return Err(format!(
                    "Previous hash mismatch at height {}: expected {}, got {}",
                    i, prev_hash, block.prev_hash
                ));
            }

            // Validate proof-of-work
            if !block.meets_target() {
                return Err(format!("Block at height {} does not meet difficulty target", i));
            }

            // Validate transactions
            for (tx_idx, tx) in block.transactions.iter().enumerate() {
                if tx_idx == 0 && tx.is_coinbase() {
                    // Coinbase is always valid
                } else {
                    match tx.is_valid(&utxo_set) {
                        Ok(()) => {}
                        Err(e) => return Err(format!(
                            "Invalid transaction at block {} tx {}: {}",
                            i, tx_idx, e
                        )),
                    }
                }
            }

            // Update UTXO set
            for tx in &block.transactions {
                if !tx.is_coinbase() {
                    for input in &tx.inputs {
                        if let Some(outputs) = utxo_set.get_mut(&input.prev_tx_id) {
                            if (input.output_index as usize) < outputs.len() {
                                outputs.remove(input.output_index as usize);
                                if outputs.is_empty() {
                                    utxo_set.remove(&input.prev_tx_id);
                                }
                            }
                        }
                    }
                }
            }

            for tx in &block.transactions {
                let tx_id = tx.id();
                let outputs: Vec<TxOutput> = tx.outputs.clone();
                utxo_set.insert(tx_id, outputs);
            }

            prev_hash = block.hash();
        }

        Ok(())
    }

    /// Get blockchain height
    pub fn height(&self) -> usize {
        self.blocks.len()
    }

    /// Get total balance for an address
    pub fn get_balance(&self, address: &str) -> u64 {
        self.utxo_set
            .values()
            .flatten()
            .filter(|output| output.address == address)
            .map(|output| output.amount)
            .sum()
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

