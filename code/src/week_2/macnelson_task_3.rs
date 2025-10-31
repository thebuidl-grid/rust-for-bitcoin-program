#[allow(dead_code)]
enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

pub fn run() {
    get_network();
    get_rpc_url(&Network::Mainnet);
}

fn get_network() {
    let network = Network::Mainnet;

    println!("What network is it?");

    match network {
        Network::Mainnet => println!("The network is Mainnet!"),
        Network::Testnet => println!("The network is Testnet!"),
        Network::Regtest => println!("The network is Regtest!"),
    }
}

fn get_rpc_url(network: &Network) -> &str {
    match network {
        Network::Mainnet => "https://alchemy.io/mainnet/starknet",
        Network::Testnet => "https://alchemy.io/testnet/starknet",
        Network::Regtest => "https://alchemy.io/regtest/starknet",
    }
}