// Task 2: Control Flow & Loops
// This demonstrates for loops, while loops, and if-else control flow

/// Simulates mining blocks with basic iteration
///
/// # Arguments
/// * `limit` - The number of blocks to mine
fn mine_blocks(limit: u8) {
    println!("=== Starting Mining Process ===\n");

    for height in 1..=limit {
        println!("Mining block #{}", height);

        // Check if we've reached a checkpoint (every 5 blocks)
        if height % 5 == 0 {
            println!("✓ Checkpoint reached at block {}", height);
        }
    }

    println!("\n=== Mining Complete ===");
}

/// Extended mining simulation with difficulty adjustment
/// Demonstrates while loops and more complex control flow
///
/// # Arguments
/// * `limit` - The number of blocks to mine
/// * `target_difficulty` - Number of attempts needed to mine each block
fn mine_blocks_with_difficulty(limit: u8, target_difficulty: u32) {
    println!(
        "=== Starting Mining with Difficulty {} ===\n",
        target_difficulty
    );

    let mut current_block = 1;

    // While loop: continue until we've mined all blocks
    while current_block <= limit {
        println!("Attempting to mine block #{}...", current_block);

        // Simulate mining attempts based on difficulty
        let mut attempts = 0;
        while attempts < target_difficulty {
            attempts += 1;
            // In real mining, this would be hash computation
        }

        println!(
            "✓ Block #{} mined after {} attempts",
            current_block, attempts
        );

        // Checkpoint notification every 5 blocks
        if current_block % 5 == 0 {
            println!("🎯 Checkpoint reached at block {}", current_block);
            println!(
                "   Progress: {}/{}  ({:.1}%)",
                current_block,
                limit,
                (current_block as f64 / limit as f64) * 100.0
            );
        }

        current_block += 1;

        // Add spacing for readability
        if current_block <= limit {
            println!();
        }
    }

    println!("\n=== All {} Blocks Mined Successfully ===", limit);
}

/// Advanced mining simulation with dynamic difficulty
/// Demonstrates nested control flow and multiple conditions
///
/// # Arguments
/// * `limit` - The number of blocks to mine
fn mine_blocks_advanced(limit: u8) {
    println!("=== Advanced Mining Simulation ===\n");

    let mut difficulty = 1;

    for height in 1..=limit {
        // Increase difficulty every 10 blocks (simulating difficulty adjustment)
        if height % 10 == 0 && height != 0 {
            difficulty += 1;
            println!("⚠ Difficulty increased to {}", difficulty);
        }

        // Simulate mining with current difficulty
        let mut attempts = 0;
        let required_attempts = difficulty * 2;

        while attempts < required_attempts {
            attempts += 1;
        }

        println!(
            "Block #{} mined (difficulty: {}, attempts: {})",
            height, difficulty, attempts
        );

        // Checkpoint every 5 blocks
        if height % 5 == 0 {
            println!("✓ Checkpoint reached at block {}", height);
        } else if height == 1 {
            println!("  Genesis block established!");
        }

        // Show progress at quarter marks
        if height == limit / 4 {
            println!("  25% complete");
        } else if height == limit / 2 {
            println!("  50% complete - halfway there!");
        } else if height == (limit * 3) / 4 {
            println!("  75% complete");
        }
    }

    println!("\n=== Mining Complete: {} blocks mined ===", limit);
}

fn main() {
    // Test basic mining function
    mine_blocks(15);

    println!("\n{}\n", "=".repeat(50));

    // Test mining with difficulty
    mine_blocks_with_difficulty(12, 3);

    println!("\n{}\n", "=".repeat(50));

    // Test advanced mining
    mine_blocks_advanced(20);
}

// Key Learning Points:
// 1. for loops with ranges: 1..=limit (inclusive)
// 2. while loops for conditional iteration
// 3. if-else statements for branching logic
// 4. Modulo operator (%) for checkpoint detection
// 5. Mutable variables with 'mut' keyword
// 6. Type casting with 'as' keyword
