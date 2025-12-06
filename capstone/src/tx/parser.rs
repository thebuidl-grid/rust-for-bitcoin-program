//! Transaction parsing utilities
//! 
//! Parse signed transactions to extract signatures and public keys for validation

use super::sighash::encode_varint;
use hex;

/// Parse a signed transaction to extract scriptSig from first input
pub fn parse_signed_transaction(tx_bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), String> {
    let mut offset = 0;
    
    // Skip version (4 bytes)
    if tx_bytes.len() < 4 {
        return Err("Transaction too short".to_string());
    }
    offset += 4;
    
    // Read input count (varint)
    let (input_count, bytes_read) = read_varint(&tx_bytes[offset..])?;
    offset += bytes_read;
    
    if input_count == 0 {
        return Err("No inputs in transaction".to_string());
    }
    
    // Read first input
    // Skip prev_tx_id (32 bytes)
    if offset + 32 > tx_bytes.len() {
        return Err("Transaction too short for prev_tx_id".to_string());
    }
    offset += 32;
    
    // Skip output_index (4 bytes)
    if offset + 4 > tx_bytes.len() {
        return Err("Transaction too short for output_index".to_string());
    }
    offset += 4;
    
    // Read scriptSig length (varint)
    let (script_sig_len, bytes_read) = read_varint(&tx_bytes[offset..])?;
    offset += bytes_read;
    
    if script_sig_len == 0 {
        return Err("Empty scriptSig".to_string());
    }
    
    // Read scriptSig
    if offset + script_sig_len as usize > tx_bytes.len() {
        return Err("Transaction too short for scriptSig".to_string());
    }
    let script_sig = tx_bytes[offset..offset + script_sig_len as usize].to_vec();
    offset += script_sig_len as usize;
    
    // Parse scriptSig: <sig_len> <sig> <sighash_byte> <pubkey_len> <pubkey>
    if script_sig.len() < 2 {
        return Err("ScriptSig too short".to_string());
    }
    
    let sig_len = script_sig[0] as usize;
    if sig_len == 0 || sig_len > script_sig.len() - 1 {
        return Err("Invalid signature length".to_string());
    }
    
    // Extract signature (without SIGHASH byte)
    let signature = script_sig[1..sig_len].to_vec();
    
    // Find pubkey (after signature + SIGHASH byte)
    let pubkey_start = sig_len + 1; // +1 for SIGHASH byte
    if pubkey_start >= script_sig.len() {
        return Err("ScriptSig too short for pubkey".to_string());
    }
    
    let pubkey_len = script_sig[pubkey_start] as usize;
    if pubkey_len == 0 || pubkey_start + 1 + pubkey_len > script_sig.len() {
        return Err("Invalid pubkey length".to_string());
    }
    
    let pubkey = script_sig[pubkey_start + 1..pubkey_start + 1 + pubkey_len].to_vec();
    
    Ok((signature, pubkey))
}

/// Read a varint from bytes (public for use in CLI)
pub fn read_varint(bytes: &[u8]) -> Result<(u64, usize), String> {
    if bytes.is_empty() {
        return Err("Empty bytes for varint".to_string());
    }
    
    match bytes[0] {
        n if n < 0xfd => Ok((n as u64, 1)),
        0xfd => {
            if bytes.len() < 3 {
                return Err("Incomplete varint".to_string());
            }
            let value = u16::from_le_bytes([bytes[1], bytes[2]]) as u64;
            Ok((value, 3))
        }
        0xfe => {
            if bytes.len() < 5 {
                return Err("Incomplete varint".to_string());
            }
            let value = u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]) as u64;
            Ok((value, 5))
        }
        0xff => {
            if bytes.len() < 9 {
                return Err("Incomplete varint".to_string());
            }
            let value = u64::from_le_bytes([
                bytes[1], bytes[2], bytes[3], bytes[4],
                bytes[5], bytes[6], bytes[7], bytes[8],
            ]);
            Ok((value, 9))
        }
        _ => Err("Invalid varint".to_string()),
    }
}

/// Extract scriptPubKey from transaction output
pub fn extract_script_pubkey_from_output(tx_bytes: &[u8], output_index: usize) -> Result<Vec<u8>, String> {
    let mut offset = 0;
    
    // Skip version
    offset += 4;
    
    // Skip inputs
    let (input_count, bytes_read) = read_varint(&tx_bytes[offset..])?;
    offset += bytes_read;
    
    for _ in 0..input_count {
        // Skip prev_tx_id
        offset += 32;
        // Skip output_index
        offset += 4;
        // Skip scriptSig
        let (script_len, bytes_read) = read_varint(&tx_bytes[offset..])?;
        offset += bytes_read + script_len as usize;
        // Skip sequence
        offset += 4;
    }
    
    // Read output count
    let (output_count, bytes_read) = read_varint(&tx_bytes[offset..])?;
    offset += bytes_read;
    
    if output_index >= output_count as usize {
        return Err("Output index out of bounds".to_string());
    }
    
    // Skip to desired output
    for i in 0..output_index {
        // Skip amount
        offset += 8;
        // Skip scriptPubKey
        let (script_len, bytes_read) = read_varint(&tx_bytes[offset..])?;
        offset += bytes_read + script_len as usize;
    }
    
    // Read amount (skip)
    offset += 8;
    
    // Read scriptPubKey
    let (script_len, bytes_read) = read_varint(&tx_bytes[offset..])?;
    offset += bytes_read;
    
    if offset + script_len as usize > tx_bytes.len() {
        return Err("Transaction too short for scriptPubKey".to_string());
    }
    
    Ok(tx_bytes[offset..offset + script_len as usize].to_vec())
}

