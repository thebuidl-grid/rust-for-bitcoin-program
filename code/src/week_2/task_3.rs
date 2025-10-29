pub enum Network {
    Mainnet,
    Testnet,
    Regtest
}

pub fn get_rpc_url(network : &Network) -> &str {
    match network {
        Network::Mainnet => "https://a_mainnet_rpc_url.com",
        Network::Testnet => "https://a_testnet_rpc_url.com",
        Network::Regtest => "https://a_regtest_rpc_url.com",
    }
}