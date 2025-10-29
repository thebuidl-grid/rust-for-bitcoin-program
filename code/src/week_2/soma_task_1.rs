// // Function that takes BTC amount and exchange rate, returns USD value
// fn btc_value_in_usd(btc: f64, rate: f64) -> f64 {
//     {
//         btc * rate
//     }
// }

// fn main() {
    
//     let btc_amount = 1.0;                         // I own 1.0 BTC
//     let exchange_rate = 65000.0;                  // Current price of BTC: $65,000 per BTC
    
//     // Calculate USD value
//     let usd_value = btc_value_in_usd(btc_amount, exchange_rate);
    
//     println!("{} BTC = ${:.2} USD", btc_amount, usd_value);
// }

// Function: btc_value_in_usd
// Takes amount of Bitcoin (btc) and current USD rate per BTC (rate)
// Returns the total value in USD using an expression block
fn btc_value_in_usd(btc: f64, rate: f64) -> f64 {
    // Expression block: the result of btc * rate is automatically returned
    // No semicolon means this is the return value
    btc * rate
}

pub fn main() {
    // Example usage with different values
    let btc_amount: f64 = 0.5;           // You own 0.5 BTC
    let usd_per_btc: f64 = 65000.0;      // Current price: $65,000 per BTC

    // Call the function
    let value_in_usd = btc_value_in_usd(btc_amount, usd_per_btc);

    // Print result with formatting
    println!("{btc_amount} BTC at ${usd_per_btc} per BTC = ${value_in_usd}");

    // Demonstrate with another value using direct expression
    println!(
        "2.3 BTC is worth ${:.2}",
        btc_value_in_usd(2.3, 68000.0)
    );

    // Show that we can use complex expressions inside the function if needed
    let portfolio_value = btc_value_in_usd(1.0, 70000.0) + btc_value_in_usd(0.25, 70000.0);
    println!("Total portfolio: ${portfolio_value}");
}