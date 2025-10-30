
fn btc_value_in_usd(btc: f64, rate: f64) -> f64 {
    btc * rate 
}

fn mine_blocks(limit: u8) {
    for height in 1..=limit {
        println!("Mining block #{}", height);
    }
}

//main run main
pub fn main() {
    let btc = 0.25; //a dream amount
    let rate = 65_000.0; // example rate here 
    let value = btc_value_in_usd(btc, rate);
    println!("{} BTC is worth ${} USD", btc, value);
    mine_blocks(10);
}
