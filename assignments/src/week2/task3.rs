// task_3.rs

/// Enum representing different Bitcoin network environments.
enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

/// This function prints details about the selected network.
fn describe_network(network: &Network) {
    match network {
        Network::Mainnet => println!("Mainnet: The live Bitcoin network."),
        Network::Testnet => println!("Testnet: For public testing."),
        Network::Regtest => println!("Regtest: For local development."),
    }
}

/// This function returns the appropriate RPC URL for each network.
fn get_rpc_url(network: &Network) -> &str {
    match network {
        Network::Mainnet => "https://mainnet.bitcoin.org",
        Network::Testnet => "https://testnet.bitcoin.org",
        Network::Regtest => "http://127.0.0.1:18443",
    }
}

fn main() {
    let current_network = Network::Regtest;

    describe_network(&current_network);
    println!("RPC URL: {}", get_rpc_url(&current_network));
}
