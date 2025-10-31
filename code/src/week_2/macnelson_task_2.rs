pub fn run() {
    // call the mine blocks function with an argument of limit 40
    mine_blocks(40); 
}

fn mine_blocks(limit: u8) {
    // This n variable is the counter of the loop
    let mut n = 1;

    // This loop runs while n is less than 20 and breaks otherwise
    while n < 20 {
        // This if statement checks if the mod of n by 5 is equal 0, only multiples of 5 will satisfy this condition therefore having us print this line every 5 steps
        if n % 5 == 0 {
            println!("Checkpoint Reached!");
        } else {
            for height in 1..=limit {
                println!("Mining block #{}", height);
            }
        }

        n += 1;
    }
}