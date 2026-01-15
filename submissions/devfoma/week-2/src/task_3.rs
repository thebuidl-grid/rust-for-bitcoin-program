// 3. Enums & Pattern Matching
// Refer to this enum `Network`
// ```
// enum Network {
//     Mainnet,
//     Testnet,
//     Regtest,
// }
// ```

// - Write a `match` block that prints details about the selected network.
// - Implement a function `fn get_rpc_url(network: &Network) -> &str` that returns different URLs.


// My Solution:

#![allow(unused)]

pub enum Network{
    Mainnet,
    Testnet,
    Regtest,
}

pub fn get_rpc_url(network: &Network) -> &str {
    match network{
        Network::Mainnet => "https://mainnet.rpc.url",
        Network::Testnet => "https://testnet.rpc.url",
        Network::Regtest => "http://localhost:18443",
    }
}