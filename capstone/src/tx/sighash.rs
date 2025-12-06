use sha2::{Sha256, Digest};
use super::builder::TransactionBuilder;

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

