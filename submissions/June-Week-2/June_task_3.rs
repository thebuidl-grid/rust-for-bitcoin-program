use crate::Network::Mainnet;
use crate::Network::Testnet;
use crate::Network::Regtest;

enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

fn get_rpc_url(network: &Network) -> &str {
    match network {
        Network::Mainnet => "mainnet.com",
        Network::Testnet => "testnet.com",
        Network::Regtest => "regtest.com",
    }
}

fn main(){
    let x = get_rpc_url(&Mainnet);
    println!("{}",x);
}