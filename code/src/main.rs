// import session_3 module
mod week_2;

// use week_2::ebuka_task_3;

use week_2::ebuka_task_3::Network;

use week_2::ebuka_task_3::get_rpc_url;

fn main() {
    //First Task
    let btc = 1.00;
    let rate = 105000.00;
    let value_in_usd = week_2::ebuka_task_1::btc_value_in_usd(btc, rate);
    println!("value in usd {}", value_in_usd);

    //Second Task
    week_2::ebuka_task_2::mine_blocks(10);

    //Third Task

    let selected_network = Network::Regtest;
    match selected_network {
        Network::Mainnet => println!("Selected: Mainnet. This is the live production network."),
        Network::Testnet => println!("Selected: Testnet. Use this for testing applications."),
        Network::Regtest => {
            println!("Selected: Regtest. This is a private network for local development.")
        }
    }
    let rpc_url = get_rpc_url(&selected_network);
    println!("RPC URL for network is: {}", rpc_url);
}
