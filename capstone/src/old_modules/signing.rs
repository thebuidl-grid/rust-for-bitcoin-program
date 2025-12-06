use secp256k1::{Secp256k1, SecretKey, PublicKey, Message, Signature, ecdsa};
use sha2::{Sha256, Digest};
use ripemd160::{Ripemd160, Digest as Ripemd160Digest};
use crate::tx_builder::{TransactionBuilder, compute_sighash, create_p2pkh_script_sig, encode_varint};
use rand::rngs::OsRng;

/// Generate a new keypair
pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    (secret_key, public_key)
}

/// Get public key hash (HASH160)
pub fn pubkey_to_hash(pubkey: &PublicKey) -> Vec<u8> {
    let pubkey_bytes = pubkey.serialize_uncompressed();
    
    // SHA256
    let mut hasher = Sha256::new();
    hasher.update(&pubkey_bytes[1..]); // Skip 0x04 prefix
    let sha256_hash = hasher.finalize();
    
    // RIPEMD160
    let mut hasher = Ripemd160::new();
    hasher.update(&sha256_hash);
    hasher.finalize().to_vec()
}

/// Sign a transaction input
pub fn sign_transaction_input(
    tx: &TransactionBuilder,
    input_index: usize,
    secret_key: &SecretKey,
    script_pubkey: &[u8],
) -> Result<Vec<u8>, String> {
    // Compute SIGHASH
    let sighash = compute_sighash(tx, input_index, script_pubkey, 0x01)?;
    
    // Create message from sighash
    let message = Message::from_digest_slice(&sighash)
        .map_err(|e| format!("Failed to create message: {}", e))?;
    
    // Sign
    let secp = Secp256k1::new();
    let signature = secp.sign_ecdsa(&message, secret_key);
    
    // Serialize signature (DER format)
    Ok(signature.serialize_compact().to_vec())
}

/// Verify a signature
pub fn verify_signature(
    sighash: &[u8],
    signature: &[u8],
    pubkey: &PublicKey,
) -> Result<bool, String> {
    let message = Message::from_digest_slice(sighash)
        .map_err(|e| format!("Failed to create message: {}", e))?;
    
    let sig = ecdsa::Signature::from_compact(signature)
        .map_err(|e| format!("Invalid signature format: {}", e))?;
    
    let secp = Secp256k1::new();
    Ok(secp.verify_ecdsa(&message, &sig, pubkey).is_ok())
}

/// Create a signed transaction
pub fn create_signed_transaction(
    tx: &TransactionBuilder,
    input_index: usize,
    secret_key: &SecretKey,
    pubkey: &PublicKey,
    script_pubkey: &[u8],
) -> Result<Vec<u8>, String> {
    // Sign the input
    let signature = sign_transaction_input(tx, input_index, secret_key, script_pubkey)?;
    
    // Create scriptSig
    let pubkey_bytes = pubkey.serialize_uncompressed();
    let script_sig = create_p2pkh_script_sig(&signature, &pubkey_bytes[1..]);
    
    // Build signed transaction
    let mut signed_tx = Vec::new();
    
    // Version
    signed_tx.extend_from_slice(&1u32.to_le_bytes());
    
    // Input count
    signed_tx.extend_from_slice(&encode_varint(tx.inputs().len() as u64));
    
    // Inputs with signatures
    for (i, input) in tx.inputs().iter().enumerate() {
        signed_tx.extend_from_slice(&input.prev_tx_id);
        signed_tx.extend_from_slice(&input.output_index.to_le_bytes());
        
        if i == input_index {
            // Include scriptSig
            signed_tx.extend_from_slice(&encode_varint(script_sig.len() as u64));
            signed_tx.extend_from_slice(&script_sig);
        } else {
            // Empty script for unsigned inputs
            signed_tx.extend_from_slice(&encode_varint(0));
        }
        
        signed_tx.extend_from_slice(&input.sequence.to_le_bytes());
    }
    
    // Outputs (same as unsigned)
    signed_tx.extend_from_slice(&encode_varint(tx.outputs().len() as u64));
    for output in tx.outputs() {
        signed_tx.extend_from_slice(&output.amount.to_le_bytes());
        signed_tx.extend_from_slice(&encode_varint(output.script_pubkey.len() as u64));
        signed_tx.extend_from_slice(&output.script_pubkey);
    }
    
    // Locktime
    signed_tx.extend_from_slice(&0u32.to_le_bytes());
    
    Ok(signed_tx)
}

