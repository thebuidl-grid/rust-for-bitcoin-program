use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a transaction input (UTXO reference)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TxInput {
    /// Previous transaction ID (hash)
    pub prev_tx_id: String,
    /// Output index in the previous transaction
    pub output_index: u32,
    /// Signature (simplified - in real Bitcoin this is a script)
    pub signature: String,
}

/// Represents a transaction output (creates a UTXO)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TxOutput {
    /// Amount in satoshis
    pub amount: u64,
    /// Recipient address (simplified - in real Bitcoin this is a script)
    pub address: String,
}

/// A Bitcoin-like transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    /// Transaction inputs
    pub inputs: Vec<TxInput>,
    /// Transaction outputs
    pub outputs: Vec<TxOutput>,
    /// Transaction timestamp
    pub timestamp: i64,
}

impl Transaction {
    /// Create a new transaction
    pub fn new(inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        Self {
            inputs,
            outputs,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    /// Calculate transaction ID (hash)
    pub fn id(&self) -> String {
        let serialized = serde_json::to_string(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        let hash = hasher.finalize();
        hex::encode(hash)
    }

    /// Validate transaction structure
    pub fn is_valid_structure(&self) -> bool {
        // Must have at least one input and one output
        if self.inputs.is_empty() || self.outputs.is_empty() {
            return false;
        }

        // All outputs must have positive amounts
        if self.outputs.iter().any(|out| out.amount == 0) {
            return false;
        }

        true
    }

    /// Check if transaction is a coinbase (mining reward)
    pub fn is_coinbase(&self) -> bool {
        // Coinbase has no inputs (or special input)
        self.inputs.is_empty()
    }

    /// Calculate total input value (requires UTXO set)
    pub fn total_input_value(&self, utxo_set: &HashMap<String, Vec<TxOutput>>) -> u64 {
        if self.is_coinbase() {
            return 0;
        }

        self.inputs
            .iter()
            .map(|input| {
                utxo_set
                    .get(&input.prev_tx_id)
                    .and_then(|outputs| outputs.get(input.output_index as usize))
                    .map(|output| output.amount)
                    .unwrap_or(0)
            })
            .sum()
    }

    /// Calculate total output value
    pub fn total_output_value(&self) -> u64 {
        self.outputs.iter().map(|out| out.amount).sum()
    }

    /// Validate transaction balances and signatures (simplified)
    pub fn is_valid(&self, utxo_set: &HashMap<String, Vec<TxOutput>>) -> Result<(), String> {
        // Check structure
        if !self.is_valid_structure() {
            return Err("Invalid transaction structure".to_string());
        }

        // Coinbase transactions are always valid (mining reward)
        if self.is_coinbase() {
            return Ok(());
        }

        // Check all inputs reference valid UTXOs
        for input in &self.inputs {
            if !utxo_set.contains_key(&input.prev_tx_id) {
                return Err(format!("Input references non-existent transaction: {}", input.prev_tx_id));
            }

            let outputs = &utxo_set[&input.prev_tx_id];
            if input.output_index as usize >= outputs.len() {
                return Err(format!(
                    "Input references invalid output index: {}",
                    input.output_index
                ));
            }
        }

        // Check that input value >= output value (no money creation)
        let input_value = self.total_input_value(utxo_set);
        let output_value = self.total_output_value();

        if input_value < output_value {
            return Err(format!(
                "Insufficient input value: {} < {}",
                input_value, output_value
            ));
        }

        // In real Bitcoin, we'd verify signatures here
        // For this simulator, we'll just check that signature is not empty
        for input in &self.inputs {
            if input.signature.is_empty() {
                return Err("Empty signature in input".to_string());
            }
        }

        Ok(())
    }
}


