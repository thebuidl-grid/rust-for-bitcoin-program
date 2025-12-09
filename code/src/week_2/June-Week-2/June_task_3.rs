use crate::Network::Mainnet; // Corrected import statement
use crate::Network::Testnet; // Corrected import statement
use crate::Network::Regtest; // Corrected import statement

enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

fn get_rpc_url(network: &Network) -> &str {
    match network { // Corrected match statement
        Network::Mainnet => "mainnet.com", // Corrected URL string
        Network::Testnet => "testnet.com",
        Network::Regtest => "regtest.com",
    }
}

fn main(){
    let x = get_rpc_url(&Mainnet); // Corrected function call
    println!("{}",x);
}