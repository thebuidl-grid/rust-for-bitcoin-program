fn main(){
    btc_value_in_usd(5.0,5.0); // Convert 5 BTC to USD at a rate of 5 USD/BTC
    println!("{x}");
}

fn btc_value_in_usd(btc: f64, rate: f64) -> f64 {
    btc * rate // Calculate the USD value
}