
pub enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

pub fn selection(network: Network) {
    match network {
        Network::Mainnet => println!("Mainnet selected"),
        Network::Testnet => println!("Testnet selected"),
        Network::Regtest => println!("Regtest selected"),
    }
}
