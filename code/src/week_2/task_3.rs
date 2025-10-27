
// Demonstrates enums, match statements, and returning string slices

enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

// Function that prints network details using match
fn print_network_info(network: &Network) {
    match network {
        Network::Mainnet => println!("You are connected to Bitcoin Mainnet "),
        Network::Testnet => println!("You are connected to Bitcoin Testnet "),
        Network::Regtest => println!("You are connected to Bitcoin Regtest "),
    }
}

// Function that returns an RPC URL for each network
fn get_rpc_url(network: &Network) -> &str {
    match network {
        Network::Mainnet => "https://mainnet.bitcoin.org/rpc",
        Network::Testnet => "https://testnet.bitcoin.org/rpc",
        Network::Regtest => "http://localhost:18443",
    }
}

fn main() {
    let network = Network::Testnet;
    print_network_info(&network);
    println!("RPC URL: {}", get_rpc_url(&network));
}
