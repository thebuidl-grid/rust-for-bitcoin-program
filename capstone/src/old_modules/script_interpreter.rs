use mini_bitcoin_node::script_example;

fn main() {
    println!("Bitcoin Script Interpreter - Project 1\n");
    println!("=====================================\n");
    
    script_example::run_all_examples();
    
    println!("\n=== Additional P2PKH Test Cases ===\n");
    
    // Test with empty signature (should fail)
    test_empty_signature();
    
    // Test with matching hash
    test_matching_hash();
}

fn test_empty_signature() {
    println!("Test: Empty signature (should fail)\n");
    
    let interpreter = mini_bitcoin_node::ScriptInterpreter::new();
    let public_key = b"test_public_key_32_bytes_long!".to_vec();
    
    // Calculate pubkey hash
    use sha2::{Sha256, Digest};
    use ripemd160::{Ripemd160, Digest as Ripemd160Digest};
    
    let mut sha256 = Sha256::new();
    sha256.update(&public_key);
    let sha256_hash = sha256.finalize();
    
    let mut ripemd160 = Ripemd160::new();
    ripemd160.update(&sha256_hash);
    let pubkey_hash = ripemd160.finalize().to_vec();
    
    let empty_sig = vec![];
    
    match interpreter.execute_p2pkh(&empty_sig, &public_key, &pubkey_hash) {
        Ok(true) => println!("✗ Empty signature was accepted (should have failed)"),
        Ok(false) => println!("✓ Empty signature correctly rejected"),
        Err(e) => println!("✓ Empty signature correctly failed: {}", e),
    }
    println!();
}

fn test_matching_hash() {
    println!("Test: Matching pubkey hash (should succeed)\n");
    
    let interpreter = mini_bitcoin_node::ScriptInterpreter::new();
    let public_key = b"another_public_key_32_bytes!".to_vec();
    
    use sha2::{Sha256, Digest};
    use ripemd160::{Ripemd160, Digest as Ripemd160Digest};
    
    let mut sha256 = Sha256::new();
    sha256.update(&public_key);
    let sha256_hash = sha256.finalize();
    
    let mut ripemd160 = Ripemd160::new();
    ripemd160.update(&sha256_hash);
    let pubkey_hash = ripemd160.finalize().to_vec();
    
    let signature = b"valid_signature_data_here".to_vec();
    
    println!("Public Key: {}", hex::encode(&public_key));
    println!("Pubkey Hash: {}", hex::encode(&pubkey_hash));
    println!();
    
    match interpreter.execute_p2pkh(&signature, &public_key, &pubkey_hash) {
        Ok(true) => println!("✓ P2PKH script with matching hash succeeded"),
        Ok(false) => println!("✗ P2PKH script failed unexpectedly"),
        Err(e) => println!("✗ P2PKH script error: {}", e),
    }
    println!();
}


