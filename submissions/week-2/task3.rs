

/// Represents the different Bitcoin network types.
enum Network {
    Mainnet,
    Testnet,
    Regtest,
}


fn get_rpc_url(network: &Network) -> &str {
    match network {
        Network::Mainnet => "https://rpc.mainnet.bitcoin.org",
        Network::Testnet => "https://rpc.testnet.bitcoin.org",
        Network::Regtest => "http://localhost:18443",
    }
}

fn main() {
    let selected_network = Network::Testnet;

    // Use match to print detailed information
    match selected_network {
        Network::Mainnet => println!("🌍 Mainnet selected — live Bitcoin network."),
        Network::Testnet => println!("🧪 Testnet selected — use for testing and development."),
        Network::Regtest => println!("⚙️ Regtest selected — local test environment."),
    }

    let rpc_url = get_rpc_url(&selected_network);
    println!("RPC endpoint: {}", rpc_url);
}
