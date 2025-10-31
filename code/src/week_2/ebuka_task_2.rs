pub fn mine_blocks(limit: u8) {
    for height in 1..=limit {
        let mut attempts = 1;
        let target_difficulty = 3;

        println!("\n--- Starting work for Block #{} ---", height);

        while attempts <= target_difficulty {
            println!("Simulating difficulty attempt #{}", attempts);
            attempts += 1;
        }
        println!("*** Block #{} Mined! ***", height);

        if height % 5 == 0 {
            println!("!> Checkpoint reached at block #{} <!", height);
        } else {
            println!("!> Continuing chain {}", height);
        }
    }
}
