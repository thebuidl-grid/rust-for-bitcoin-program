fn mine_blocks(limit: u8) {
    for height in 1..=limit {
        println!("Mining block #{}", height);

        // E.g, Each block requires 50 more attempts than its height number
        let target = height as u32 * 50;
        // Attempts for each block starts at 0
        let mut attempts = 0;

        while attempts < target {
            attempts += 1;
        }

        println!("Block mined after {} attempts", attempts);

        // Print checkpoint after every 5 blocks
        if height % 5 == 0 {
            println!("CHECKPOINT REACHED!\n")
        }
    }
}

pub fn main() {
    mine_blocks(28);
}
