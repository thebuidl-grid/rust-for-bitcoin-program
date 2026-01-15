fn mine_blocks(limit: u8) {
    let mut height = 1;
    let mut difficulty = 1;

    // Standard for loop to iterate over blocks
    for h in 1..=limit {
        println!("Mining block #{}", h);

        // Print checkpoint every 5 blocks
        if h % 5 == 0 {
            println!("✅ Checkpoint reached at block #{}", h);
        }
    }

    println!("\n--- Simulating difficulty increase ---");
    // Simulating difficulty using a while loop
    while difficulty <= 3 {
        println!("Mining with difficulty level {}", difficulty);
        difficulty += 1;
    }

}

fn main() {
    mine_blocks(12);
}