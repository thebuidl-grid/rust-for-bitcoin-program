/// Task 2: Control Flow & Loops
/// 
/// This task demonstrates:
/// - For loops for block mining simulation
/// - While loops for difficulty simulation
/// - If-else blocks for checkpoints
/// - Pattern matching with ranges

/// Simulates mining blocks with difficulty and checkpoints
/// 
/// # Arguments
/// * `limit` - The maximum block height to mine
fn mine_blocks(limit: u8) {
    println!("Starting block mining simulation...\n");
    
    for height in 1..=limit {
        println!("Mining block #{}", height);
        
        // Checkpoint every 5 blocks
        if height % 5 == 0 {
            println!("✓ Checkpoint reached at block #{}\n", height);
        }
    }
}

/// Simulates mining with difficulty adjustment using a while loop
/// 
/// # Arguments
/// * `target_blocks` - The number of blocks to mine
fn mine_blocks_with_difficulty(target_blocks: u8) {
    println!("\n=== Mining with Difficulty Simulation ===\n");
    
    let mut height = 1;
    let mut difficulty = 1;
    
    while height <= target_blocks {
        println!("Mining block #{} (Difficulty: {})", height, difficulty);
        
        // Increase difficulty every 10 blocks
        if height % 10 == 0 {
            difficulty += 1;
            println!("⚡ Difficulty increased to {}\n", difficulty);
        }
        
        height += 1;
    }
}

fn main() {
    println!("=== Task 2: Control Flow & Loops ===\n");
    
    // Basic mining simulation with checkpoints
    mine_blocks(20);
    
    // Mining with difficulty adjustment
    mine_blocks_with_difficulty(25);
    
    println!("\n=== Mining Summary ===");
    println!("Total blocks mined: 20 (basic) + 25 (with difficulty)");
}

