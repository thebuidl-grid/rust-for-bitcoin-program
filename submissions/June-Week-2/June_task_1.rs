fn main(){
    btc_value_in_usd(5.0,5.0);
    println!("{x}");
}

fn btc_value_in_usd(btc: f64, rate: f64) -> f64 {
    btc * rate
}