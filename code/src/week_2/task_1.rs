fn btc_value_in_usd(btc: f64, rate: f64) -> f64 {
    btc * rate
}
pub fn main() {
    let btc = 1.2;
    let rate = 100000.00;
    let value = btc_value_in_usd(btc, rate);
    println!("{} BTC is worth ${}", btc, value);
}
