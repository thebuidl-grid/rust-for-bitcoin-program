pub fn run() {
    // call the btc_value_in_usd function with some arguments
    let value = btc_value_in_usd(5.4, 20000.0);
    println!("BTC value in USD: {}", value);
}


/// This function btc_value_in_usd has two parameters - btc and rate.
pub fn btc_value_in_usd(btc: f64, rate: f64) -> f64 {
    // On receipt of the btc and rate, an operation is performed on them to get the btc value in usd
    btc * rate
}