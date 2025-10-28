#![allow(unused)]
enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

fn print_network_details(network: &Network) {
    match network {
        Network::Mainnet => println!(
            "Network: Bitcoin Mainnet\nType: Production network with real BTC\nPort: 28391"
        ),
        Network::Testnet => {
            println!("Network: Bitcoin Testnet\nType: Test network with test BTC\nPort: 28231")
        }
        Network::Regtest => println!(
            "Network: Bitcoin Regtest\nType: Regression test network with test BTC\nPort: 28211"
        ),
    }
}

fn get_rpc_url(network: &Network) -> &str {
    match network {
        Network::Mainnet => "https://bitcoinrpc:mainnet:user1:12345",
        Network::Testnet => "https://bitcoinrpc:testnet:user1:12345",
        Network::Regtest => "https://bitcoinrpc:regtest:user1:12345",
    }
}

pub fn main() {
    let network_1 = Network::Mainnet;
    let network_2 = Network::Regtest;

    print_network_details(&network_1);
    println!("{}", get_rpc_url(&network_2));
}
