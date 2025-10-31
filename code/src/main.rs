use crate::kelly_musk::week2::task_3;
use crate::kelly_musk::week2::task_2;
use crate::kelly_musk::week2::task_1;
use crate::kelly_musk::week2::task_3::get_rpc_url;
use crate::task_3::Network;
// import session_3 module
mod week_2;
mod kelly_musk;
// entry point to run all topics covered
fn main() {
    // call functions from session_3 modules
    week_2::function::run();
    week_2::loops::run();
    week_2::primitive_types::run();

    task_1::bitcoin_value_in_usd(7, 11500);
    task_2::mine_blocks(101);

    
    let regtest_network_url  = Network::Regtest;
    task_3::get_rpc_url(&regtest_network_url);
    println!("regtest network url {:?}", get_rpc_url(&regtest_network_url));

    let mainnet_network_url = Network::Mainnet;
    task_3::get_rpc_url(&mainnet_network_url);
    println!("mainnet network url {:?}", get_rpc_url(&mainnet_network_url));

    let testnent_network_url = Network::Testnet;
    task_3::get_rpc_url(&testnent_network_url);
    println!("testnet network url {:?}", get_rpc_url(&testnent_network_url))
}