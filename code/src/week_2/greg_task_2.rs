pub fn mine_blocks(limit: u8) {
    let mut difficulty = 1;

    for height in 1..=limit {
        println!("Mining block #{}", height);
        while difficulty < 5 {
            difficulty += 1
        }

        if height % 5 == 0 {
            println!("Checkpoint reached");
        } else {
            let block_after_checkpoint=height % 5;
            println!("blocks after check Point #{block_after_checkpoint}");
        }
    }
}
