pub fn mine_blocks(limit: u8) {
    let mut height = 1;
    let mut difficulty = 1;
    
    while height <= limit {
        println!("Mining block #{}, Difficulty: {}", height, difficulty);
        
        // Check for checkpoints every 5 blocks
        if height % 5 == 0 {
            println!("Checkpoint reached!");
            
            // Increase difficulty after each checkpoint
            difficulty += 1;
        }
        
        height += 1;
    }
}