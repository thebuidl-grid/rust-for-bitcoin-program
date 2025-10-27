fn mine_blocks(limit: u8) {
    let mut difficulty = 1;

    for height in 1..=limit {
        println!("Mining block #{}", height);

        // Simulate difficulty
        while difficulty < 3 {
            println!("Simulating difficulty level {}", difficulty);
            difficulty += 1; // the number of starting zero's
        }

        if height % 5 == 0 {
            println!("Checkpoint reached");
        }
    }
}

fn main() {
    mine_blocks(10);
}
