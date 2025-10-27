# Week 2 — Rust Fundamentals: 10 Hands-on Exercises
Build strong Rust fundamentals while preparing for Bitcoin-focused development.

#### 1. Functions and Expressions
- Write a function `btc_value_in_usd(btc: f64, rate: f64) -> f64` that returns btc * rate.

Demonstrate:
- Function return types
- Expression blocks (no semicolons for return)

#### 2. Control Flow & Loops
Write a function that simulates mining blocks:

```
fn mine_blocks(limit: u8) {
    for height in 1..=limit {
        println!("Mining block #{}", height);
    }
}
```
- Extend it using:
    - A while loop to simulate difficulty
    - An if-else block to print “Checkpoint reached” every 5 blocks.

#### 3. Enums & Pattern Matching
Refer to this enum `Network`
```
enum Network {
    Mainnet,
    Testnet,
    Regtest,
}
```

- Write a `match` block that prints details about the selected network.
- Implement a function `fn get_rpc_url(network: &Network) -> &str` that returns different URLs.



# How to Submit Your Assignment
Follow these steps carefully to submit your work:
- Fork this repository to your own GitHub account.
- Each of the above tasks should be treated as a separate task eg: 1 is `task_1.rs`
- Create a new branch for your submission:
`git checkout -b week-2-assignment`
- Add your task inside the appropriate `week_2` folder:
```
week_2/
    ├── task_1.rs/
    ├── task_2.rs/
    ├── task_3.rs/

```
- Commit and push your changes:
```
git add .
git commit -m "chore: week 2 assignment"
git push origin week-2-assignment
```

- Create a Pull Request (PR) to the main repository

# Evaluation Criteria
Submissions will be evaluated based on:
- Completeness and correctness of your assignment
- Clarity of documentation (notes and code comments)
- Consistency in following submission structure
