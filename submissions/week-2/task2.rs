fn mine_blocks(limit: u8) {
    let mut mined = 0;

    while mined < limit {
        mined += 1;
        println!("Mining block #{}", mined);

        // Simulate mining difficulty — fake pause or iteration logic could go here.
        // Example: adjusting difficulty every few blocks.
        if mined % 5 == 0 {
            println!("⛓️  Checkpoint reached at block #{}", mined);
        } else {
            println!("...continuing to next block");
        }
    }

    println!("✅ Mining complete — total {} blocks mined!", mined);
}

fn main() {
    mine_blocks(12);
}
