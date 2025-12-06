//! Bitcoin Transaction CLI
//! 
//! Complete workflow: Build -> Sign -> Verify transactions using the Script Interpreter

use clap::{Parser, Subcommand};
use btc_script_tx::*;
use std::fs;
use hex;

#[derive(Parser)]
#[command(name = "btc-tx")]
#[command(about = "Bitcoin Transaction Builder & Script Interpreter CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new keypair
    GenKey {
        /// Output file for private key
        #[arg(short, long, default_value = "privkey.hex")]
        privkey_file: String,
        /// Output file for public key
        #[arg(short, long, default_value = "pubkey.hex")]
        pubkey_file: String,
    },
    /// Build an unsigned transaction
    BuildTx {
        /// Previous transaction ID (hex, big-endian)
        #[arg(short, long)]
        from_tx: String,
        /// Output index
        #[arg(short, long)]
        output_index: u32,
        /// Recipient pubkey hash (hex)
        #[arg(short, long)]
        to: String,
        /// Amount in satoshis
        #[arg(short, long)]
        amount: u64,
        /// Output file
        #[arg(short, long, default_value = "unsigned_tx.hex")]
        output: String,
    },
    /// Sign a transaction
    SignTx {
        /// Unsigned transaction file
        #[arg(short, long)]
        tx_file: String,
        /// Private key (hex)
        #[arg(short, long)]
        privkey: String,
        /// Previous transaction ID
        #[arg(short, long)]
        from_tx: String,
        /// Output index
        #[arg(short, long)]
        output_index: u32,
        /// Output file
        #[arg(short, long, default_value = "signed_tx.hex")]
        output: String,
    },
    /// Verify a transaction with Script Interpreter
    VerifyTx {
        /// Signed transaction file
        #[arg(short, long)]
        tx_file: String,
        /// Public key (hex)
        #[arg(short, long)]
        pubkey: String,
        /// Expected pubkey hash (hex)
        #[arg(short, long)]
        pubkey_hash: String,
    },
    /// Complete workflow: Build, Sign, and Verify
    Workflow {
        /// Previous transaction ID (for demo, use all zeros)
        #[arg(short, long, default_value = "0000000000000000000000000000000000000000000000000000000000000000")]
        from_tx: String,
        /// Output index
        #[arg(short, long, default_value = "0")]
        output_index: u32,
        /// Amount in satoshis
        #[arg(short, long, default_value = "50000")]
        amount: u64,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenKey { privkey_file, pubkey_file } => {
            println!("Generating keypair...");
            let (secret_key, public_key) = generate_keypair();
            let pubkey_hash = pubkey_to_hash(&public_key);
            
            // Save private key
            let privkey_hex = hex::encode(secret_key.secret_bytes());
            fs::write(&privkey_file, &privkey_hex)?;
            
            // Save public key
            let pubkey_bytes = public_key.serialize_uncompressed();
            let pubkey_hex = hex::encode(&pubkey_bytes[1..]); // Skip 0x04
            fs::write(&pubkey_file, &pubkey_hex)?;
            
            println!("✓ Private key saved to: {}", privkey_file);
            println!("✓ Public key saved to: {}", pubkey_file);
            println!("Public key hash: {}", hex::encode(&pubkey_hash));
        }
        
        Commands::BuildTx { from_tx, output_index, to, amount, output } => {
            println!("Building unsigned transaction...");
            
            let prev_tx_bytes = tx::hex_to_le_bytes(&from_tx)?;
            let pubkey_hash = hex::decode(&to)?;
            
            let script_pubkey = create_p2pkh_script_pubkey(&pubkey_hash);
            
            let tx = TransactionBuilder::new()
                .add_input(UnsignedInput {
                    prev_tx_id: prev_tx_bytes,
                    output_index,
                    script_pubkey: script_pubkey.clone(),
                    sequence: 0xffffffff,
                })
                .add_output(Output {
                    amount,
                    script_pubkey,
                });
            
            let hex = tx.to_hex();
            fs::write(&output, &hex)?;
            
            println!("✓ Unsigned transaction saved to: {}", output);
            println!("Transaction hex: {}", hex);
        }
        
        Commands::SignTx { tx_file, privkey, from_tx, output_index, output } => {
            println!("Signing transaction...");
            
            // Parse private key
            let privkey_bytes = hex::decode(&privkey)?;
            let secp = secp256k1::Secp256k1::new();
            let secret_key = secp256k1::SecretKey::from_slice(&privkey_bytes)
                .map_err(|e| format!("Invalid private key: {}", e))?;
            let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
            
            // Get pubkey hash
            let pubkey_hash = pubkey_to_hash(&public_key);
            let script_pubkey = create_p2pkh_script_pubkey(&pubkey_hash);
            
            // Rebuild transaction (in real implementation, parse unsigned tx)
            let prev_tx_bytes = tx::hex_to_le_bytes(&from_tx)?;
            let tx = TransactionBuilder::new()
                .add_input(UnsignedInput {
                    prev_tx_id: prev_tx_bytes,
                    output_index,
                    script_pubkey: script_pubkey.clone(),
                    sequence: 0xffffffff,
                });
            
            // Sign
            let signed_tx = create_signed_transaction(&tx, 0, &secret_key, &public_key, &script_pubkey)?;
            let signed_hex = hex::encode(&signed_tx);
            
            fs::write(&output, &signed_hex)?;
            println!("✓ Signed transaction saved to: {}", output);
            println!("Transaction hex: {}", signed_hex);
        }
        
        Commands::VerifyTx { tx_file, pubkey: _pubkey, pubkey_hash } => {
            println!("Verifying transaction with Script Interpreter...\n");
            
            // Read and parse signed transaction
            let tx_hex = fs::read_to_string(&tx_file)?;
            let tx_bytes = hex::decode(tx_hex.trim())?;
            
            // Extract signature and pubkey from transaction
            let (signature, pubkey_from_tx) = parse_signed_transaction(&tx_bytes)
                .map_err(|e| format!("Failed to parse transaction: {}", e))?;
            
            println!("Extracted from transaction:");
            println!("  Signature: {} bytes", signature.len());
            println!("  Public key: {} bytes\n", pubkey_from_tx.len());
            
            // Get expected hash
            let expected_hash = hex::decode(&pubkey_hash)?;
            
            // Reconstruct the unsigned transaction for SIGHASH computation
            // (In a real implementation, we'd parse the full transaction structure)
            // For now, we'll use the expected scriptPubKey
            let script_pubkey = create_p2pkh_script_pubkey(&expected_hash);
            
            // Extract prev_tx_id and output_index from transaction
            // Skip version (4 bytes)
            let mut offset = 4;
            
            // Read input count (varint)
            let (input_count, bytes_read) = read_varint(&tx_bytes[offset..])
                .map_err(|e| format!("Failed to read input count: {}", e))?;
            offset += bytes_read;
            
            if input_count == 0 {
                return Err("No inputs in transaction".into());
            }
            
            // Extract prev_tx_id (32 bytes, little-endian)
            if offset + 32 > tx_bytes.len() {
                return Err("Transaction too short for prev_tx_id".into());
            }
            let prev_tx_id = tx_bytes[offset..offset + 32].to_vec();
            offset += 32;
            
            // Extract output_index (4 bytes, little-endian)
            if offset + 4 > tx_bytes.len() {
                return Err("Transaction too short for output_index".into());
            }
            let output_index = u32::from_le_bytes([
                tx_bytes[offset],
                tx_bytes[offset + 1],
                tx_bytes[offset + 2],
                tx_bytes[offset + 3],
            ]);
            
            // Build unsigned transaction for SIGHASH
            let unsigned_tx = TransactionBuilder::new()
                .add_input(UnsignedInput {
                    prev_tx_id,
                    output_index,
                    script_pubkey: script_pubkey.clone(),
                    sequence: 0xffffffff,
                });
            
            // Compute SIGHASH
            let sighash = compute_sighash(&unsigned_tx, 0, &script_pubkey, 0x01)
                .map_err(|e| format!("Failed to compute SIGHASH: {}", e))?;
            
            // Verify with Script Interpreter
            let interpreter = ScriptInterpreter::new();
            
            println!("Executing P2PKH script validation...");
            match interpreter.execute_p2pkh(&signature, &pubkey_from_tx, &expected_hash, Some(&sighash)) {
                Ok(true) => {
                    println!("\n✓ Transaction signature is VALID!");
                    println!("✓ Pubkey hash verification: PASSED");
                    println!("✓ Script Interpreter validation: PASSED");
                    println!("\nTransaction is valid and ready for broadcast!");
                }
                Ok(false) => {
                    println!("\n✗ Transaction signature is INVALID");
                    println!("✗ Script Interpreter validation: FAILED");
                }
                Err(e) => {
                    println!("\n✗ Verification error: {}", e);
                    println!("✗ Script Interpreter validation: FAILED");
                }
            }
        }
        
        Commands::Workflow { from_tx, output_index, amount } => {
            println!("=== Complete Transaction Workflow ===\n");
            
            // 1. Generate keypair
            println!("1. Generating keypair...");
            let (secret_key, public_key) = generate_keypair();
            let pubkey_hash = pubkey_to_hash(&public_key);
            let pubkey_bytes = public_key.serialize_uncompressed();
            
            println!("   Private key: {}", hex::encode(secret_key.secret_bytes()));
            println!("   Public key: {}", hex::encode(&pubkey_bytes[1..]));
            println!("   Pubkey hash: {}\n", hex::encode(&pubkey_hash));
            
            // 2. Build transaction
            println!("2. Building unsigned transaction...");
            let prev_tx_bytes = tx::hex_to_le_bytes(&from_tx)?;
            let script_pubkey = create_p2pkh_script_pubkey(&pubkey_hash);
            
            let tx = TransactionBuilder::new()
                .add_input(UnsignedInput {
                    prev_tx_id: prev_tx_bytes,
                    output_index,
                    script_pubkey: script_pubkey.clone(),
                    sequence: 0xffffffff,
                })
                .add_output(Output {
                    amount,
                    script_pubkey: script_pubkey.clone(),
                });
            
            let unsigned_hex = tx.to_hex();
            println!("   Unsigned TX: {}\n", unsigned_hex);
            
            // 3. Sign transaction
            println!("3. Signing transaction...");
            let signed_tx = create_signed_transaction(&tx, 0, &secret_key, &public_key, &script_pubkey)?;
            let signed_hex = hex::encode(&signed_tx);
            println!("   Signed TX: {}\n", signed_hex);
            
            // 4. Verify with Script Interpreter (parse the signed transaction)
            println!("4. Verifying signed transaction with Script Interpreter...");
            
            // Parse the signed transaction to extract signature and pubkey
            let (signature, pubkey_from_tx) = parse_signed_transaction(&signed_tx)
                .map_err(|e| format!("Failed to parse signed transaction: {}", e))?;
            
            // Compute SIGHASH for verification
            let sighash = compute_sighash(&tx, 0, &script_pubkey, 0x01)?;
            
            // Verify with Script Interpreter
            let interpreter = ScriptInterpreter::new();
            match interpreter.execute_p2pkh(&signature, &pubkey_from_tx, &pubkey_hash, Some(&sighash)) {
                Ok(true) => {
                    println!("   ✓ Signature verification: VALID");
                    println!("   ✓ Pubkey hash verification: VALID");
                    println!("   ✓ Script execution: SUCCESS");
                    println!("   ✓ Transaction validation: PASSED\n");
                    println!("=== Transaction is VALID and ready for broadcast! ===");
                    println!("\nFinal transaction hex:");
                    println!("{}", signed_hex);
                }
                Ok(false) => {
                    println!("   ✗ Transaction validation FAILED\n");
                }
                Err(e) => {
                    println!("   ✗ Validation error: {}\n", e);
                }
            }
        }
    }
    
    Ok(())
}

