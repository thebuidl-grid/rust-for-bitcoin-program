pub fn mine_blocks(limit: u8) {
    let mut difficulty = 1;

    
    for height in 1..=limit {
        println!("Mining block #{}", height);

        while difficulty < 1{
            println!("Simulating dificulty {}", difficulty);
            difficulty += 1 ;
        }

        if height % 5 == 0 {
            println!("Checkpoint reached")
        }
    }
}