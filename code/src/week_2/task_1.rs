// Demonstrates functions, return types, and expression blocks

// Function that converts BTC to USD
fn btc_value_in_usd(btc: f64, rate: f64) -> f64 {
    // The expression below returns btc * rate without using 'return' or a semicolon
    btc * rate
}

fn main() {
    let btc = 0.05;
    let rate = 68_000.0; // Example: 1 BTC = $68,000
    let value = btc_value_in_usd(btc, rate);
    println!("{} BTC is worth ${} USD", btc, value);
}
