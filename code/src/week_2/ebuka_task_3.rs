pub enum Network {
    Mainnet,
    Testnet,
    Regtest,
}
pub fn get_rpc_url(network: &Network) -> &str {
    match network {
        Network::Mainnet => "https://api.mainnet.com/rpc",
        Network::Testnet => "https://testnet-explorer.io/rpc",
        Network::Regtest => "http://localhost:18443/rpc",
    }
}
