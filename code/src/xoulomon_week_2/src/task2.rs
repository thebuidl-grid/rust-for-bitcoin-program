pub fn mine_blocks(limit: u8) {
    let mut difficulty = 1;

    for height in 0..limit {
        let block_no = height + 1;
        println!("Mining block #{block_no}");

        while difficulty < height {
            difficulty += 1;
            println!("Simluating difficulty {difficulty}");  
            println!("  ");                
        }

        if height % 5 == 0 {
            println!("Checkpoint reached ✅");
            println!("  ");
        }

    }
}