#[derive(Debug)]
pub enum Network {
    Mainnet,
    Testnet,
    Regtest
}

pub fn print_network_details(network: &Network) {
    match network {
        Network::Mainnet=> println!("You are running a Mainnet Network"),
        Network::Regtest=> println!("You are running a Regtest Network"),
        Network::Testnet=> println!("You are running a Testnet Network")
    }
}

pub fn get_rpc_url(network: &Network) -> &str {
    match network {
        Network::Mainnet=> "https://mainnet.com",
        Network::Regtest=> "https://regtest.com",
        Network::Testnet=> "https://0.0.1.8332"
    }
}