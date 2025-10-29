// Function that takes BTC amount and exchange rate, returns USD value
fn btc_value_in_usd(btc: f64, rate: f64) -> f64 {
    {
        btc * rate
    }
}

pub fn main() {
    
    let btc_amount = 1.0;                         // I own 1.0 BTC
    let exchange_rate = 65000.0;                  // Current exchange_rate of BTC: $65,000 per BTC
    
    // Calculate USD value
    let usd_value = btc_value_in_usd(btc_amount, exchange_rate);
    
    println!("{} BTC = ${:.2} USD", btc_amount, usd_value);
}

