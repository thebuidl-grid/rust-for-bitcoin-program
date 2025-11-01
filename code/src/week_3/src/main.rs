use clap::{Arg, Command};
use week_3::decode_transaction;

fn main() {
    // Define CLI using Clap
    let matches = Command::new("Transaction Decoder")
        .version("1.0")
        .about("Bitcoin Transaction Decoder CLI")
        .arg(
            Arg::new("transaction_hex")
                .required(true)
                .help("(string, required) Raw transaction hex"),
        )
        .get_matches();

    // Retrieve transaction hex argument
    let transaction_hex = matches
        .get_one::<String>("transaction_hex")
        .expect("Transaction hex is required")
        .to_string();

    // Call the decoder function from the library
    match week_3::decode_transaction(transaction_hex) {
        Ok(transaction) => println!("Decoded Transaction:\n{}", transaction),
        Err(e) => eprintln!("Error: {}", e),
    }
}

// // https://mempool.space/testnet/tx/3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2