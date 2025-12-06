use crate::script::{ScriptInterpreter, ScriptContext};
use sha2::{Sha256, Digest};
use ripemd160::{Ripemd160, Digest as Ripemd160Digest};
use hex;

/// Example: Valid P2PKH script execution
pub fn example_valid_p2pkh() {
    println!("=== Example: Valid P2PKH Script ===\n");

    let interpreter = ScriptInterpreter::new();
    
    // Simulate a public key and its hash
    let public_key = b"fake_public_key_32_bytes_long!!".to_vec();
    
    // Calculate pubkey hash (HASH160 = RIPEMD160(SHA256(pubkey)))
    let mut sha256 = Sha256::new();
    sha256.update(&public_key);
    let sha256_hash = sha256.finalize();
    
    let mut ripemd160 = Ripemd160::new();
    ripemd160.update(&sha256_hash);
    let pubkey_hash = ripemd160.finalize().to_vec();
    
    let signature = b"fake_signature_data".to_vec();
    
    println!("Public Key: {}", hex::encode(&public_key));
    println!("Pubkey Hash: {}\n", hex::encode(&pubkey_hash));
    
    match interpreter.execute_p2pkh(&signature, &public_key, &pubkey_hash) {
        Ok(true) => println!("\n✓ P2PKH script executed successfully!"),
        Ok(false) => println!("\n✗ P2PKH script returned false"),
        Err(e) => println!("\n✗ P2PKH script failed: {}", e),
    }
}

/// Example: Invalid P2PKH script (wrong pubkey hash)
pub fn example_invalid_p2pkh() {
    println!("\n=== Example: Invalid P2PKH Script (Wrong Hash) ===\n");

    let interpreter = ScriptInterpreter::new();
    
    let public_key = b"fake_public_key_32_bytes_long!!".to_vec();
    let wrong_pubkey_hash = b"wrong_hash_20_bytes!!".to_vec(); // Wrong hash!
    let signature = b"fake_signature_data".to_vec();
    
    println!("Public Key: {}", hex::encode(&public_key));
    println!("Wrong Pubkey Hash: {}\n", hex::encode(&wrong_pubkey_hash));
    
    match interpreter.execute_p2pkh(&signature, &public_key, &wrong_pubkey_hash) {
        Ok(true) => println!("\n✗ Script should have failed but didn't!"),
        Ok(false) => println!("\n✓ Script correctly returned false"),
        Err(e) => println!("\n✓ Script correctly failed: {}", e),
    }
}

/// Example: Custom script execution
pub fn example_custom_script() {
    println!("\n=== Example: Custom Script Execution ===\n");

    let interpreter = ScriptInterpreter::new();
    let mut context = ScriptContext::new();
    
    // Script: push data, duplicate, hash160
    use crate::script::Opcode;
    let script = vec![
        Opcode::OP_PUSHDATA(b"hello world".to_vec()),
        Opcode::OP_DUP,
        Opcode::OP_HASH160,
    ];
    
    println!("Script: PUSH 'hello world', DUP, HASH160\n");
    
    match interpreter.execute(&script, &mut context) {
        Ok(result) => {
            println!("\nScript result: {}", result);
            println!("Stack size: {}", context.stack.len());
            if let Some(top) = context.stack.back() {
                println!("Top of stack: {}", hex::encode(top));
            }
        }
        Err(e) => println!("\nScript failed: {}", e),
    }
}

/// Run all script examples
pub fn run_all_examples() {
    example_valid_p2pkh();
    example_invalid_p2pkh();
    example_custom_script();
}

