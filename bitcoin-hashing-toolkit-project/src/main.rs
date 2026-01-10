use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use bitcoin_do::{sha256, ripemd160, hash160, generate_keypair, derive_address};
use secp256k1::PublicKey;
use hex;

#[derive(Parser)]
#[command(name = "bitcoin-do")]
#[command(about = "This tool provides essential cryptograpic functions for Bitcoin development:
- Hashing functions: SHA256, RIPEMD160, HASH160
- Key generation: secp256k1 keypairs
- Address derivation: P2PKH Bitcoin addresses

Examples:
  Generate a new keypair:
    bitcoin-do generate-key

  Derive address from public key:
    bitcoin-do derive-address 02...

  Hash some data:
    bitcoin-do sha256 68656c6c6f")]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Compute SHA256 hash of hex data
    Sha256 { data: String },
    // Compute RIPEMD160 hash of hex data
    Ripemd160 { data: String },
    // Compute HASH160 (RIPEMD160 of SHA256) of hex dat
    Hash160 { data: String },
    // Generate a new secp256k1 keypair
    GenerateKey,
    // Derive Bitcoin address from public key (hex)
    DeriveAddress { pubkey: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sha256 { data } => {
            match hex::decode(&data) {
                Ok(bytes) => {
                    let hash = sha256(&bytes);
                    println!("Your SHA256: {}", hash.green());
                }
                Err(_) => eprintln!("{}", "Invalid hex data".red()), 
            }
        }
        Commands::Ripemd160 { data } => {
            match hex::decode(&data) {
                Ok(bytes) => {
                    let hash = ripemd160(&bytes);
                    println!("Your RIPEMD160: {}", hash.green());
                }
                Err(_) => eprintln!("{}", "Invalid hex data".red()),
            }
        }
        Commands::Hash160 { data } => {
            match hex::decode(&data) {
                Ok(bytes) => {
                    let hash = hash160(&bytes);
                    println!("Your HASH160: {}", hash.green());
                }
                Err(_) => eprintln!("{}", "Invalid hex data".red()),
            }
        }
        Commands::GenerateKey => {
            let (priv_key, pub_key) = generate_keypair();
            println!("Your Bitcoin Private Key: {}", hex::encode(priv_key.secret_bytes()).yellow());
            println!("Your Bitcoin Public Key: {}", hex::encode(pub_key.serialize()).cyan());
        }
        Commands::DeriveAddress { pubkey } => {
            match hex::decode(&pubkey) {
                Ok(bytes) => {
                    match PublicKey::from_slice(&bytes){
                        Ok(pk) => {
                            let address = derive_address(&pk);
                            println!("Your  Bitcoin Address: {}", address.magenta());
                        }
                        Err(_) => eprintln!("{}", "Invalid public key".red()),
                    }
                }
                Err(_) => eprintln!("{}", "Invalid hex public key".red()),
            }
        }
    }
}
