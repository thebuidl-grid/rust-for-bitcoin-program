fn btc_value_in_usd(btc: f64, rate:f64) -> f64{
    btc * rate
}

fn main() {
    let btc_amount = 0.05;
    let usd_rate = 67000.0;

    let value = btc_value_in_usd(btc_amount, usd_rate);
    println!("{} BTC = ${} USD", btc_amount, value);
}