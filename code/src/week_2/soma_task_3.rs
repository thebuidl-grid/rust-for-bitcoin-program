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

fn print_network_details(network: &Network) {
    match network {
        Network::Mainnet => println!("This is the main Bitcoin network."),
        Network::Testnet => println!("This is the test Bitcoin network."),
        Network::Regtest => println!("This is the regtest Bitcoin network."),
    }
}

pub fn main() {
    let network = Network::Regtest;

    print_network_details(&network);
    println!("RPC URL: {}", get_rpc_url(&network));
    let network = Network::Mainnet;

    print_network_details(&network);
    println!("RPC URL: {}", get_rpc_url(&network));
    let network = Network::Testnet;

    print_network_details(&network);
    println!("RPC URL: {}", get_rpc_url(&network));
}
