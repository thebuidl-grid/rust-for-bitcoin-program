fn main(){
    mine_blocks(15, 50);
}

fn mine_blocks(limit: u32, difficulty: u64) {
    for height in 1..=limit {
        println!("Mining block #{}", height);

        let mut work = 0;
        while work < difficulty {
            work += 1;
        }

        if height % 5 == 0 {
            println!("Mined block #{}. CHECKPOINT REACHED!!!! YAYYY", height);
        } else {
            println!("Mined block #{}", height);
        }
    }
}