pub enum Network {
    Mainnet,
    Testnet,
    Regtest
}

fn get_rpc_url(network: &Network) -> &str {
    match network {
        Network::Mainnet => "https://Mainnet.com",
        Network::Testnet => "https://Testnet.com",
        Network::Regtest => "https://Regtest.com"
    }
}

fn print_network_info(network: &Network) {
    match network {
        Network::Mainnet => println!("your are running the mainnet bitcoin network"),
        Network::Testnet => println!("your are running the testnet bitcoin network"),
        Network::Regtest => println!("your are running the regtest bitcoin network")
    }
}

fn main() {
    println!("task 3");

    let mainnetnetwork = Network::Mainnet;
    print_network_info(&mainnetnetwork);
    println!("and its rps url is {}", get_rpc_url(&mainnetnetwork));

    let testnetnetwork = Network::Testnet;
    print_network_info(&testnetnetwork);
    println!("and its rpc url is {}", get_rpc_url(&testnetnetwork));

    let regtestnetwork = Network::Regtest;
    print_network_info(&regtestnetwork);
    println!("and it's rpc url is {}", get_rpc_url(&regtestnetwork));
}