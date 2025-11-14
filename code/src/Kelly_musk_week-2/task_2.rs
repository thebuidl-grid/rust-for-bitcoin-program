

pub fn mine_blocks(limit: u8) {
    let difficulty = 1;


    for height in 1..=limit {
        println!("Mining block #{}", height);

            while difficulty < 3 {
                println!("Simulating difficulty level {}", difficulty);
                difficulty += 1;
            }

            if height % 5 ==  0 {
                println!("checkpoint reached")
            }
            
    }
}

fn main() {
    println!("task 2");
    mine_blocks(30);
}