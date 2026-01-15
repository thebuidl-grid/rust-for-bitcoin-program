//  1. Functions and Expressions
// - Write a function `btc_value_in_usd(btc: f64, rate: f64) -> f64` that returns btc * rate.

// Demonstrate:
// - Function return types
// - Expression blocks (no semicolons for return)
// made the function public to be accessible from main.rs

// My Solution:
#![allow(unused)]

pub fn btc_value_in_usd(btc: f64, rate: f64) -> f64 {
    btc * rate
}
