use crate::transaction::Transaction;
use std::collections::{HashMap, HashSet};

/// Local mempool for pending transactions
#[derive(Debug, Clone)]
pub struct Mempool {
    /// Pending transactions indexed by transaction ID
    transactions: HashMap<String, Transaction>,
    /// Set of spent UTXOs (to prevent double-spending)
    spent_utxos: HashSet<(String, u32)>,
}

impl Mempool {
    /// Create a new empty mempool
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
            spent_utxos: HashSet::new(),
        }
    }

    /// Add a transaction to the mempool
    pub fn add_transaction(
        &mut self,
        tx: Transaction,
        utxo_set: &HashMap<String, Vec<crate::transaction::TxOutput>>,
    ) -> Result<(), String> {
        // Validate transaction
        tx.is_valid(utxo_set)?;

        // Check for double-spending within mempool
        if !tx.is_coinbase() {
            for input in &tx.inputs {
                let utxo_key = (input.prev_tx_id.clone(), input.output_index);
                if self.spent_utxos.contains(&utxo_key) {
                    return Err(format!(
                        "Double-spend detected: UTXO {}:{} already spent in mempool",
                        input.prev_tx_id, input.output_index
                    ));
                }
            }
        }

        let tx_id = tx.id();

        // Add to mempool
        self.transactions.insert(tx_id.clone(), tx.clone());

        // Mark UTXOs as spent
        if !tx.is_coinbase() {
            for input in &tx.inputs {
                self.spent_utxos.insert((input.prev_tx_id.clone(), input.output_index));
            }
        }

        Ok(())
    }

    /// Remove transactions from mempool (when included in a block)
    pub fn remove_transactions(&mut self, tx_ids: &[String]) {
        for tx_id in tx_ids {
            if let Some(tx) = self.transactions.remove(tx_id) {
                // Unmark UTXOs as spent
                if !tx.is_coinbase() {
                    for input in &tx.inputs {
                        self.spent_utxos.remove(&(input.prev_tx_id.clone(), input.output_index));
                    }
                }
            }
        }
    }

    /// Get all pending transactions
    pub fn get_transactions(&self) -> Vec<&Transaction> {
        self.transactions.values().collect()
    }

    /// Get transactions for mining (select up to a limit)
    pub fn get_transactions_for_block(&self, max_count: usize) -> Vec<Transaction> {
        self.transactions
            .values()
            .take(max_count)
            .cloned()
            .collect()
    }

    /// Check if mempool is empty
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    /// Get number of pending transactions
    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    /// Clear all transactions from mempool
    pub fn clear(&mut self) {
        self.transactions.clear();
        self.spent_utxos.clear();
    }
}

impl Default for Mempool {
    fn default() -> Self {
        Self::new()
    }
}


