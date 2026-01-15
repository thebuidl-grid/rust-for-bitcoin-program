mod task1;
mod task2;
mod task3;

use task1::btc_value_in_usd;
use task2::mine_blocks;
use task3::{Network, selection};


fn main() {

    println!("Task 1");
    let btc_val = 0.7;
    let rate = 110_111.75;
    let btc_usd = btc_value_in_usd(btc_val, rate);
    println!("The price of {btc_val} in usd is {btc_usd}");

    println!("Task 2");
    mine_blocks(10);

    println!("Task 3");
    let _mainnet = Network::Mainnet;
    let _testnet = Network::Testnet;
    let _regtest = Network::Regtest;
    selection(_regtest);





}