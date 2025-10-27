
// Demonstrates control flow, for loops, while loops, and conditionals

fn mine_blocks(limit: u8) {
    let mut difficulty = 1;

    for height in 1..=limit {
        println!("Mining block #{} at difficulty {}", height, difficulty);

        // Every 5 blocks, simulate a checkpoint
        if height % 5 == 0 {
            println!("Checkpoint reached at block #{}", height);
        }

        // Simulate difficulty increase using a while loop
        let mut work_done = 0;
        while work_done < difficulty {
            println!("Performing mining work... (step {})", work_done + 1);
            work_done += 1;
        }

        difficulty += 1; // Increase difficulty for next round
    }
}

fn main() {
    mine_blocks(10);
}
