pub fn mine_blocks(limit: u8) {
    for height in 1..=limit {
        println!("Mining block #{}", height);

        let mut difficulty = 0;
        while difficulty < 3 { 
            println!("  Working... tr #{}", difficulty + 1);
            difficulty += 1;
        }

        if height % 5 == 0 {
            println!("--- Checkpoint reached at block {} ---", height);
        }

        println!(); 
    }
}
