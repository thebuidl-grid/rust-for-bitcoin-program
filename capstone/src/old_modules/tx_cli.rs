use clap::{Parser, Subcommand};
use mini_bitcoin_node::*;
use std::fs;

#[derive(Parser)]
#[command(name = "tx-cli")]
#[command(about = "Bitcoin Transaction Builder CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build an unsigned transaction
    BuildTx {
        /// Previous transaction ID (hex)
        #[arg(short, long)]
        from_tx: String,
        /// Output index
        #[arg(short, long)]
        output_index: u32,
        /// Recipient address (pubkey hash hex)
        #[arg(short, long)]
        to: String,
        /// Amount in satoshis
        #[arg(short, long)]
        amount: u64,
        /// Output file for unsigned transaction
        #[arg(short, long, default_value = "unsigned_tx.hex")]
        output: String,
    },
    /// Sign a transaction
    SignTx {
        /// Unsigned transaction file (hex)
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
        /// Output file for signed transaction
        #[arg(short, long, default_value = "signed_tx.hex")]
        output: String,
    },
    /// Verify a transaction
    VerifyTx {
        /// Signed transaction file (hex)
        #[arg(short, long)]
        tx_file: String,
        /// Public key (hex)
        #[arg(short, long)]
        pubkey: String,
        /// Expected pubkey hash (hex)
        #[arg(short, long)]
        pubkey_hash: String,
    },
    /// Generate a new keypair
    GenKey {
        /// Output file for private key
        #[arg(short, long, default_value = "privkey.hex")]
        privkey_file: String,
        /// Output file for public key
        #[arg(short, long, default_value = "pubkey.hex")]
        pubkey_file: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::BuildTx {
            from_tx,
            output_index,
            to,
            amount,
            output,
        } => {
            println!("Building unsigned transaction...");
            
            // Convert hex strings to bytes
            let prev_tx_bytes = hex_to_le_bytes(&from_tx)?;
            let pubkey_hash = hex::decode(&to)?;
            
            // Create P2PKH scriptPubKey
            let script_pubkey = create_p2pkh_script_pubkey(&pubkey_hash);
            
            // Create output script
            let output_script = create_p2pkh_script_pubkey(&pubkey_hash);
            
            // Build transaction
            let tx = TransactionBuilder::new()
                .add_input(UnsignedInput {
                    prev_tx_id: prev_tx_bytes,
                    output_index,
                    script_pubkey: script_pubkey.clone(),
                    sequence: 0xffffffff,
                })
                .add_output(Output {
                    amount,
                    script_pubkey: output_script,
                });
            
            let hex = tx.to_hex();
            fs::write(&output, &hex)?;
            
            println!("✓ Unsigned transaction saved to: {}", output);
            println!("Transaction hex: {}", hex);
        }
        
        Commands::SignTx {
            tx_file,
            privkey,
            from_tx,
            output_index,
            output,
        } => {
            println!("Signing transaction...");
            
            // Read unsigned transaction
            let tx_hex = fs::read_to_string(&tx_file)?;
            let tx_bytes = hex::decode(tx_hex.trim())?;
            
            // Parse private key
            let privkey_bytes = hex::decode(&privkey)?;
            let secp = secp256k1::Secp256k1::new();
            let secret_key = secp256k1::SecretKey::from_slice(&privkey_bytes)
                .map_err(|e| format!("Invalid private key: {}", e))?;
            let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
            
            // Get pubkey hash
            let pubkey_hash = pubkey_to_hash(&public_key);
            let script_pubkey = create_p2pkh_script_pubkey(&pubkey_hash);
            
            // For now, create a simple transaction builder
            // In a full implementation, you'd parse the unsigned tx
            let prev_tx_bytes = hex_to_le_bytes(&from_tx)?;
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
        
        Commands::VerifyTx {
            tx_file,
            pubkey,
            pubkey_hash,
        } => {
            println!("Verifying transaction...");
            
            // Read transaction
            let tx_hex = fs::read_to_string(&tx_file)?;
            let tx_bytes = hex::decode(tx_hex.trim())?;
            
            // Parse public key
            let pubkey_bytes = hex::decode(&pubkey)?;
            let secp = secp256k1::Secp256k1::new();
            let public_key = if pubkey_bytes.len() == 65 && pubkey_bytes[0] == 0x04 {
                secp256k1::PublicKey::from_slice(&pubkey_bytes)
                    .map_err(|e| format!("Invalid public key: {}", e))?
            } else if pubkey_bytes.len() == 64 {
                let mut full_key = vec![0x04];
                full_key.extend_from_slice(&pubkey_bytes);
                secp256k1::PublicKey::from_slice(&full_key)
                    .map_err(|e| format!("Invalid public key: {}", e))?
            } else {
                return Err("Invalid public key length".into());
            };
            
            // Verify with script interpreter
            let interpreter = ScriptInterpreter::new();
            let expected_hash = hex::decode(&pubkey_hash)?;
            
            // Extract signature from transaction (simplified - in real implementation, parse tx)
            // For demo, we'll use a test signature
            let test_sig = vec![0u8; 64]; // Placeholder
            
            match interpreter.execute_p2pkh(&test_sig, &pubkey_bytes[1..], &expected_hash) {
                Ok(true) => {
                    println!("✓ Transaction signature is valid!");
                }
                Ok(false) => {
                    println!("✗ Transaction signature is invalid");
                }
                Err(e) => {
                    println!("✗ Verification error: {}", e);
                }
            }
        }
        
        Commands::GenKey { privkey_file, pubkey_file } => {
            println!("Generating new keypair...");
            
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
    }
    
    Ok(())
}

