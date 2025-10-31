enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

fn get_rpc_url(network: &Network) -> &str {
    match network {
        Network::Mainnet => "https://localhost:8332",
        Network::Testnet => "https://localhost:18332",
        Network::Regtest => "https://localhost:18443",
    }
}

fn main() {
    let networks = [Network::Mainnet, Network::Testnet, Network::Regtest];
    
    for network in networks.iter() {
        match network {
            Network::Mainnet => println!("Connected to Bitcoin Mainnet - Main Environment"),
            Network::Testnet => println!("Connected to Bitcoin Testnet - Testing Environment"),
            Network::Regtest => println!("Connected to Bitcoin Regtest - Local Development"),
        }
        println!("RPC URL: {}", get_rpc_url(network));
        println!("-------------------");
    }
}

