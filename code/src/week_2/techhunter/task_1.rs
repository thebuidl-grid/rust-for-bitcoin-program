// Task 1: Functions and Expressions
// This demonstrates function return types and expression blocks in Rust

/// Converts Bitcoin value to USD
///
/// # Arguments
/// * `btc` - Amount of Bitcoin
/// * `rate` - Current BTC/USD exchange rate
///
/// # Returns
/// * `f64` - The USD value of the Bitcoin amount
///
/// # Example
/// ```
/// let usd_value = btc_value_in_usd(0.5, 45000.0);
/// println!("Value: ${}", usd_value); // Output: Value: $22500
/// ```
fn btc_value_in_usd(btc: f64, rate: f64) -> f64 {
    // Expression block - no semicolon means this value is returned
    btc * rate
}

fn main() {
    // Test the function with different values
    let btc_amount = 1.5;
    let exchange_rate = 43500.50;

    // Call the function and store result
    let usd_value = btc_value_in_usd(btc_amount, exchange_rate);

    println!("Bitcoin Amount: {} BTC", btc_amount);
    println!("Exchange Rate: ${:.2}/BTC", exchange_rate);
    println!("USD Value: ${:.2}", usd_value);

    println!("\n--- Additional Examples ---");

    // Example with different amounts
    println!(
        "0.01 BTC at $45,000/BTC = ${:.2}",
        btc_value_in_usd(0.01, 45000.0)
    );
    println!(
        "2.5 BTC at $42,000/BTC = ${:.2}",
        btc_value_in_usd(2.5, 42000.0)
    );
    println!(
        "10 BTC at $50,000/BTC = ${:.2}",
        btc_value_in_usd(10.0, 50000.0)
    );
}

// Key Learning Points:
// 1. Function signature specifies parameter types and return type
// 2. Expression without semicolon is automatically returned
// 3. f64 is used for decimal numbers (floating point)
// 4. The last expression in a function is the return value
