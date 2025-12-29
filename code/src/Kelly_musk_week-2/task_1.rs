pub fn btc_value_in_usd(btc: f64, rate: f64) -> f64 {
    ubtc * rate
}


fn main() {
    println!("task_1");
    let btc = 1.2;
    let rate = 115_194.75;
    let btcrate = btc_value_in_usd(btc, rate);
    println!("this the rate of bitcoin in usd $ {} in the year 2025", btcrate);
}