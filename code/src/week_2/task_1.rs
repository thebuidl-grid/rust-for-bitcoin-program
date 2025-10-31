fn btc_value_in_usd(btc: f64, rate: f64) -> f64 {
    btc * rate
}
//main run main
fn main() {
    let btc = 0.25; //a dream amount
    let rate = 65_000.0; // example rate here 
    let value = btc_value_in_usd(btc, rate);
    println!("{} BTC is worth ${} USD", btc, value);
}
