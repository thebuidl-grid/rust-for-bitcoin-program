use crate::transaction::{Transaction, TxInput, TxOutput};
use sha2::{Sha256, Digest};
use std::io::Write;

/// Transaction builder for creating raw unsigned transactions
pub struct TransactionBuilder {
    version: u32,
    inputs: Vec<UnsignedInput>,
    outputs: Vec<Output>,
    locktime: u32,
}

/// Unsigned transaction input
#[derive(Debug, Clone)]
pub struct UnsignedInput {
    pub prev_tx_id: Vec<u8>,  // 32 bytes, little-endian
    pub output_index: u32,
    pub script_pubkey: Vec<u8>, // Locking script
    pub sequence: u32,
}

/// Transaction output
#[derive(Debug, Clone)]
pub struct Output {
    pub amount: u64,        // Satoshis
    pub script_pubkey: Vec<u8>, // Locking script
}

impl TransactionBuilder {
    /// Create a new transaction builder
    pub fn new() -> Self {
        Self {
            version: 1,
            inputs: Vec::new(),
            outputs: Vec::new(),
            locktime: 0,
        }
    }

    /// Add an input
    pub fn add_input(mut self, input: UnsignedInput) -> Self {
        self.inputs.push(input);
        self
    }

    /// Add an output
    pub fn add_output(mut self, output: Output) -> Self {
        self.outputs.push(output);
        self
    }

    /// Set locktime
    pub fn set_locktime(mut self, locktime: u32) -> Self {
        self.locktime = locktime;
        self
    }

    /// Build unsigned transaction (returns raw hex)
    pub fn build_unsigned(&self) -> Vec<u8> {
        let mut tx = Vec::new();
        
        // Version (4 bytes, little-endian)
        tx.extend_from_slice(&self.version.to_le_bytes());
        
        // Input count (varint)
        tx.extend_from_slice(&encode_varint(self.inputs.len() as u64));
        
        // Inputs
        for input in &self.inputs {
            // Previous transaction ID (32 bytes, little-endian)
            tx.extend_from_slice(&input.prev_tx_id);
            
            // Output index (4 bytes, little-endian)
            tx.extend_from_slice(&input.output_index.to_le_bytes());
            
            // Script length (varint) - empty for unsigned
            tx.extend_from_slice(&encode_varint(0));
            
            // Sequence (4 bytes, little-endian)
            tx.extend_from_slice(&input.sequence.to_le_bytes());
        }
        
        // Output count (varint)
        tx.extend_from_slice(&encode_varint(self.outputs.len() as u64));
        
        // Outputs
        for output in &self.outputs {
            // Amount (8 bytes, little-endian)
            tx.extend_from_slice(&output.amount.to_le_bytes());
            
            // Script length (varint)
            tx.extend_from_slice(&encode_varint(output.script_pubkey.len() as u64));
            
            // Script
            tx.extend_from_slice(&output.script_pubkey);
        }
        
        // Locktime (4 bytes, little-endian)
        tx.extend_from_slice(&self.locktime.to_le_bytes());
        
        tx
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.build_unsigned())
    }

    /// Get inputs (for signing)
    pub fn inputs(&self) -> &[UnsignedInput] {
        &self.inputs
    }

    /// Get outputs
    pub fn outputs(&self) -> &[Output] {
        &self.outputs
    }
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Encode variable-length integer (varint)
pub fn encode_varint(n: u64) -> Vec<u8> {
    if n < 0xfd {
        vec![n as u8]
    } else if n <= 0xffff {
        let mut v = vec![0xfd];
        v.extend_from_slice(&(n as u16).to_le_bytes());
        v
    } else if n <= 0xffffffff {
        let mut v = vec![0xfe];
        v.extend_from_slice(&(n as u32).to_le_bytes());
        v
    } else {
        let mut v = vec![0xff];
        v.extend_from_slice(&n.to_le_bytes());
        v
    }
}

/// Compute SIGHASH for a transaction input
pub fn compute_sighash(
    tx: &TransactionBuilder,
    input_index: usize,
    script_pubkey: &[u8],
    sighash_type: u32,
) -> Result<Vec<u8>, String> {
    if input_index >= tx.inputs().len() {
        return Err("Input index out of bounds".to_string());
    }

    // Simplified SIGHASH_ALL implementation
    // Real Bitcoin SIGHASH is more complex
    
    let mut preimage = Vec::new();
    
    // Version
    preimage.extend_from_slice(&1u32.to_le_bytes());
    
    // Input count
    preimage.extend_from_slice(&encode_varint(tx.inputs().len() as u64));
    
    // All inputs (with script for the one being signed)
    for (i, input) in tx.inputs().iter().enumerate() {
        preimage.extend_from_slice(&input.prev_tx_id);
        preimage.extend_from_slice(&input.output_index.to_le_bytes());
        
        if i == input_index {
            // Include the script for the input being signed
            preimage.extend_from_slice(&encode_varint(script_pubkey.len() as u64));
            preimage.extend_from_slice(script_pubkey);
        } else {
            // Empty script for other inputs
            preimage.extend_from_slice(&encode_varint(0));
        }
        
        preimage.extend_from_slice(&input.sequence.to_le_bytes());
    }
    
    // Output count
    preimage.extend_from_slice(&encode_varint(tx.outputs().len() as u64));
    
    // All outputs
    for output in tx.outputs() {
        preimage.extend_from_slice(&output.amount.to_le_bytes());
        preimage.extend_from_slice(&encode_varint(output.script_pubkey.len() as u64));
        preimage.extend_from_slice(&output.script_pubkey);
    }
    
    // Locktime
    preimage.extend_from_slice(&0u32.to_le_bytes());
    
    // SIGHASH type
    preimage.extend_from_slice(&sighash_type.to_le_bytes());
    
    // Double SHA256
    let mut hasher = Sha256::new();
    hasher.update(&preimage);
    let hash1 = hasher.finalize();
    
    let mut hasher2 = Sha256::new();
    hasher2.update(&hash1);
    let hash2 = hasher2.finalize();
    
    Ok(hash2.to_vec())
}

/// Create P2PKH scriptPubKey from pubkey hash
pub fn create_p2pkh_script_pubkey(pubkey_hash: &[u8]) -> Vec<u8> {
    // OP_DUP OP_HASH160 <20-byte-pubkey-hash> OP_EQUALVERIFY OP_CHECKSIG
    let mut script = Vec::new();
    script.push(0x76); // OP_DUP
    script.push(0xa9); // OP_HASH160
    script.push(0x14); // Push 20 bytes
    script.extend_from_slice(pubkey_hash);
    script.push(0x88); // OP_EQUALVERIFY
    script.push(0xac); // OP_CHECKSIG
    script
}

/// Create P2PKH scriptSig from signature and public key
pub fn create_p2pkh_script_sig(signature: &[u8], pubkey: &[u8]) -> Vec<u8> {
    let mut script = Vec::new();
    
    // Push signature (with SIGHASH_ALL byte)
    script.push(signature.len() as u8 + 1);
    script.extend_from_slice(signature);
    script.push(0x01); // SIGHASH_ALL
    
    // Push public key
    script.push(pubkey.len() as u8);
    script.extend_from_slice(pubkey);
    
    script
}

/// Convert hex string to little-endian bytes (for txid)
pub fn hex_to_le_bytes(hex: &str) -> Result<Vec<u8>, String> {
    let bytes = hex::decode(hex)
        .map_err(|e| format!("Invalid hex: {}", e))?;
    
    // Reverse for little-endian
    Ok(bytes.into_iter().rev().collect())
}

