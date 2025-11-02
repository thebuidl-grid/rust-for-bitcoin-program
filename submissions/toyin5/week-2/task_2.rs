fn mine_blocks(limit: u8) {
    for height in 1..=limit {
        println!("Mining block #{}", height);
        let difficulty: u64 = (height as u64 / 2).saturating_add(2);
        println!("Difficulty set to {}", difficulty);

        
        let mut attempts: u64 = 1;
        while attempts % difficulty != 0 {
            attempts += 1;
        }
        println!("Block #{} mined after {} attempts", height, attempts);

        if is_a_product_of_5(height as u32) {
            println!("Block #{}. Checkpoint reached", height);
        }
    }
}

fn main() {
    let block_limit = 10;
    mine_blocks(block_limit);
}

fn is_a_product_of_5(n: u32) -> bool {
    n % 5 == 0
}