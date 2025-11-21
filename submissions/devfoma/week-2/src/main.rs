mod task_1;
mod task_2;
mod task_3;

fn main() {
    // Example usage of task_1 function
    let btc_amount = 0.5;
    let exchange_rate = 30000.0; // Example exchange rate: 1 BTC = 30,000 USD

    // calling the function from task_1 module
    let usd_value = task_1::btc_value_in_usd(btc_amount, exchange_rate);
    println!("The value of {} BTC in USD is: ${}", btc_amount, usd_value);

    // Example usage of task_2 function
    task_2::mine_blocks(12);

    // Example usage of task_3 function
    let rpc_url = task_3::get_rpc_url(&task_3::Network::Mainnet);
    println!("RPC URL for Mainnet: {}", rpc_url);

    
}
