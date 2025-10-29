/// Task 1: Functions and Expressions
/// 
/// This task demonstrates:
/// - Function return types
/// - Expression blocks (no semicolons for return)
/// - Converting BTC to USD using a given exchange rate

/// Converts Bitcoin value to USD
/// 
/// # Arguments
/// * `btc` - The amount of Bitcoin
/// * `rate` - The exchange rate (BTC to USD)
/// 
/// # Returns
/// The equivalent value in USD
fn btc_value_in_usd(btc: f64, rate: f64) -> f64 {
    btc * rate
}

fn main() {
    println!("=== Task 1: Functions and Expressions ===\n");
    
    // Example 1: Convert 1 BTC to USD at $45,000 per BTC
    let btc_amount = 1.0;
    let exchange_rate = 45000.0;
    let usd_value = btc_value_in_usd(btc_amount, exchange_rate);
    
    println!("Bitcoin Amount: {} BTC", btc_amount);
    println!("Exchange Rate: ${} per BTC", exchange_rate);
    println!("USD Value: ${}\n", usd_value);
    
    // Example 2: Convert 0.5 BTC to USD at $50,000 per BTC
    let btc_amount_2 = 0.5;
    let exchange_rate_2 = 50000.0;
    let usd_value_2 = btc_value_in_usd(btc_amount_2, exchange_rate_2);
    
    println!("Bitcoin Amount: {} BTC", btc_amount_2);
    println!("Exchange Rate: ${} per BTC", exchange_rate_2);
    println!("USD Value: ${}\n", usd_value_2);
    
    // Example 3: Convert 2.5 BTC to USD at $48,500 per BTC
    let btc_amount_3 = 2.5;
    let exchange_rate_3 = 48500.0;
    let usd_value_3 = btc_value_in_usd(btc_amount_3, exchange_rate_3);
    
    println!("Bitcoin Amount: {} BTC", btc_amount_3);
    println!("Exchange Rate: ${} per BTC", exchange_rate_3);
    println!("USD Value: ${}", usd_value_3);
}

