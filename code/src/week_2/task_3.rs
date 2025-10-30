enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

fn get_rpc_url(network: &Network) -> &str {
    match network {
        Network::Mainnet => "https://mainnet.example.com",
        Network::Testnet => "https://testnet.example.com",
        Network::Regtest => "http://localhost:8332",
    }
}

pub fn main() {

    let mut network = Network::Regtest;
    println!("Net: {}", get_rpc_url(&network));

    network = Network::Mainnet;
    println!("Net: {}", get_rpc_url(&network));

    network = Network::Testnet;
    println!("Net: {}", get_rpc_url(&network));
    
}
