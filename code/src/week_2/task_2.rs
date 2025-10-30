use core::time;

fn mine_blocks(limit: u8) {
    let mut difficulty = 1;

    for height in 1..=limit {
        println!("Mining block #{}", height);

        let time_to_mine =  difficulty * 1000;

        while difficulty < 5 {
            println!("Simulating difficulty level {}", difficulty);
            // Simulate mining time
            std::thread::sleep(std::time::Duration::from_millis(time_to_mine));
            break;
        }

        if height % 5 == 0 {
            println!("Checkpoint reached");
        }

        difficulty += 1; 

    }
}

pub fn main() {
    mine_blocks(21);
}
