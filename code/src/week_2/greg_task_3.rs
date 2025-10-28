pub enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

pub fn get_rpc_url(network: &Network) -> &str {
    match network {
        Network::Mainnet => "https//btc_mainnet_rpc.com",
        Network::Regtest => "https//btc_Regtest_rpc.com",
        Network::Testnet => "https//btc_testnet_rpc.com",
    }
}
