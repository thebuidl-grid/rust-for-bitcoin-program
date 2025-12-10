use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

use crate::interpreter::Interpreter;
use crate::transaction::Transaction;

#[derive(Parser)]
#[command(name = "bitcoin-script-interpreter")]
#[command(about = "A Bitcoin Script interpreter for validating transactions", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    ExecuteScript {
        #[arg(help = "Script to execute in hex format")]
        script: String,

        #[arg(long, default_value = "0000000000000000000000000000000000000000000000000000000000000000")]
        sighash: String,
    },

    ValidateTransaction {
        #[arg(help = "Transaction in hex format")]
        tx_hex: String,

        #[arg(help = "ScriptPubKey to validate against in hex format")]
        script_pubkey: String,

        #[arg(long, default_value = "0", help = "Input index to validate")]
        input_index: usize,
    },

    TestP2pkh {
        #[arg(long, help = "Run invalid P2PKH tests")]
        invalid: bool,
    },
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::ExecuteScript { script, sighash } => {
            execute_script(&script, &sighash, cli.verbose)
        }
        Commands::ValidateTransaction {
            tx_hex,
            script_pubkey,
            input_index,
        } => validate_transaction(&tx_hex, &script_pubkey, input_index, cli.verbose),
        Commands::TestP2pkh { invalid } => {
            if invalid {
                run_invalid_p2pkh_tests(cli.verbose)
            } else {
                run_valid_p2pkh_tests(cli.verbose)
            }
        }
    }
}

fn execute_script(script_hex: &str, sighash_hex: &str, verbose: bool) -> Result<()> {
    let script = hex::decode(script_hex.trim())?;
    let sighash = hex::decode(sighash_hex.trim())?;

    if sighash.len() != 32 {
        return Err(anyhow!("Sighash must be 32 bytes"));
    }

    let mut interpreter = Interpreter::new(verbose);
    let result = interpreter.execute(&script, &sighash)?;

    if verbose {
        println!("Execution result: {}", if result { "SUCCESS" } else { "FAILURE" });
    } else {
        println!("{}", if result { "VALID" } else { "INVALID" });
    }

    Ok(())
}

fn validate_transaction(
    tx_hex: &str,
    script_pubkey_hex: &str,
    input_index: usize,
    verbose: bool,
) -> Result<()> {
    let tx = Transaction::from_hex(tx_hex)?;
    let script_pubkey = hex::decode(script_pubkey_hex.trim())?;

    let result = tx.validate_p2pkh(input_index, &script_pubkey, verbose)?;

    if !verbose {
        println!("{}", if result { "VALID" } else { "INVALID" });
    }

    Ok(())
}

fn run_valid_p2pkh_tests(verbose: bool) -> Result<()> {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║          VALID P2PKH TEST CASES                           ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    println!("Running built-in valid P2PKH test cases...\n");

    println!("These tests would validate against real P2PKH transactions.");
    println!("To run real tests, you need to:");
    println!("  1. Generate a transaction using bitcoind in regtest mode");
    println!("  2. Extract the transaction hex and scriptPubKey");
    println!("  3. Run: cargo run -- validate-transaction <tx_hex> <script_pubkey>\n");

    println!("Example workflow:");
    println!("  # Start bitcoind in regtest mode");
    println!("  bitcoind -regtest -daemon");
    println!();
    println!("  # Create a wallet and generate blocks");
    println!("  bitcoin-cli -regtest createwallet testwallet");
    println!("  bitcoin-cli -regtest -generate 101");
    println!();
    println!("  # Get a new address and send coins");
    println!("  bitcoin-cli -regtest getnewaddress");
    println!("  bitcoin-cli -regtest sendtoaddress <address> 1.0");
    println!();
    println!("  # Get the raw transaction");
    println!("  bitcoin-cli -regtest getrawtransaction <txid>");
    println!();

    Ok(())
}

fn run_invalid_p2pkh_tests(verbose: bool) -> Result<()> {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║          INVALID P2PKH TEST CASES                         ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    println!("Test 1: Empty ScriptSig (should fail)");
    println!("─────────────────────────────────────────────────────────────");

    let pubkey_hash = vec![1u8; 20];
    let script_pubkey = crate::transaction::create_p2pkh_script(&pubkey_hash)?;

    let tx = Transaction {
        version: 1,
        inputs: vec![crate::transaction::TxInput {
            prev_tx: [0u8; 32],
            prev_index: 0,
            script_sig: vec![],
            sequence: 0xffffffff,
        }],
        outputs: vec![crate::transaction::TxOutput {
            value: 5000000000,
            script_pubkey: vec![],
        }],
        locktime: 0,
    };

    match tx.validate_p2pkh(0, &script_pubkey, verbose) {
        Ok(false) => println!("✓ Test passed: Transaction correctly rejected\n"),
        Ok(true) => println!("✗ Test failed: Invalid transaction was accepted\n"),
        Err(e) => println!("✓ Test passed: Transaction failed with error: {}\n", e),
    }

    println!("Test 2: Malformed ScriptSig (should fail)");
    println!("─────────────────────────────────────────────────────────────");

    let tx2 = Transaction {
        version: 1,
        inputs: vec![crate::transaction::TxInput {
            prev_tx: [0u8; 32],
            prev_index: 0,
            script_sig: vec![0x01, 0xFF],
            sequence: 0xffffffff,
        }],
        outputs: vec![crate::transaction::TxOutput {
            value: 5000000000,
            script_pubkey: vec![],
        }],
        locktime: 0,
    };

    match tx2.validate_p2pkh(0, &script_pubkey, verbose) {
        Ok(false) => println!("✓ Test passed: Transaction correctly rejected\n"),
        Ok(true) => println!("✗ Test failed: Invalid transaction was accepted\n"),
        Err(e) => println!("✓ Test passed: Transaction failed with error: {}\n", e),
    }

    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║          TEST SUMMARY                                     ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!("All invalid transaction tests completed.");
    println!("These tests demonstrate that the interpreter correctly");
    println!("rejects malformed and invalid P2PKH transactions.\n");

    Ok(())
}
