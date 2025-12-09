fn main(){
    mine_blocks(15, 50); // Mine 15 blocks with a difficulty of 50
}

fn mine_blocks(limit: u32, difficulty: u64) {
    for height in 1..=limit {
        println!("Mining block #{}", height);

        let mut work = 0; //counter to simulate work done
        while work < difficulty { // Simulate mining work
            work += 1; // Increment work done
        }

        if height % 5 == 0 { // Checkpoint every 5 blocks
            println!("Mined block #{}. CHECKPOINT REACHED!!!! YAYYY", height);
        } else {
            println!("Mined block #{}", height);
        }
    }
}