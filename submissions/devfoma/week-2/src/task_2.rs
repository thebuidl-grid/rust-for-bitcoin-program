// 2. Control Flow & Loops
// Write a function that simulates mining blocks:

// ```
// fn mine_blocks(limit: u8) {
//     for height in 1..=limit {
//         println!("Mining block #{}", height);
//     }
// }
// ```
// - Extend it using:
//     - A while loop to simulate difficulty
//     - An if-else block to print “Checkpoint reached” every 5 blocks.

// My solution:
#![allow(unused)]

pub fn mine_blocks(limit: u8) {
    let mut height = 1;
    while height <= limit {
        println!("Mining block #{}", height);
        if height % 5 == 0 {
            println!("Checkpoint reached at block #{}", height);
        }
        height += 1;
    }
}