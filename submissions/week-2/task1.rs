
fn btc_value_in_usd(btc: f64, rate: f64) -> f64 {
    // Expression block without a semicolon to return the computed value.
    {
        btc * rate
    }
}

fn main() {
    let btc = 0.25;
    let rate = 65000.0;
    let value_in_usd = btc_value_in_usd(btc, rate);

    println!(
        "{} BTC at rate ${} = ${} USD",
        btc, rate, value_in_usd
    );
}
