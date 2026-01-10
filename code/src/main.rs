mod task1;
mod task2;
mod task3;

use task1::btc_value_in_usd;
use task2::mine_blocks;
use task3::{get_rpc_url, Network };


fn main() {
    println!("Hello, world!");

    println!("Task 1");
    let btc_val = 0.5;
    let rate = 115_194.75;
    let btc_usd = btc_value_in_usd(btc_val, rate);
    println!("The price of {btc_val} in usd is {btc_usd}");

    println!("Task 2");
    mine_blocks(15);

    println!("Task 3");
    let network = Network::Mainnet;
    let rpc = get_rpc_url(&network);
    println!("The rpc of mainnet is {rpc}");

    let network = Network::Testnet;
    let rpc = get_rpc_url(&network);
    println!("The rpc of testnet is {rpc}");


// import week_2 module
mod week_2;
// entry point to run all topics covered
fn main() {
    // call functions from session_3 modules
    // week_2::function::run();
    // week_2::loops::run();
    // week_2::primitive_types::run();

    // run assignments functions
    week_2::task_1::main();
    week_2::task_2::main();
    week_2::task_3::main();
}