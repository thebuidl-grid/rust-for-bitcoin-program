mod script_type;
mod script_impl;
mod opcodes;
mod hashing;

use std::io::{self, Write};
use colored::*;

use script_impl::{Script};


fn main() {
    println!("{}", "\nBit_Check".yellow().bold());
    println!("{}", "==========================\n".black());

    print!("{}", "Locking Script: ".blue().bold());
    io::stdout().flush().unwrap();
    let mut locking_input = String::new();
    io::stdin().read_line(&mut locking_input).unwrap();
    let locking_script = Script::new(locking_input.trim());
    println!("Type: {:?}\n", locking_script.script_type);

    print!("{}", "Unlocking Script: ".blue().bold());
    io::stdout().flush().unwrap();
    let mut unlocking_input = String::new();
    io::stdin().read_line(&mut unlocking_input).unwrap();
    let unlocking_script = Script::new(unlocking_input.trim());

    println!("\nLocking Script: {}", locking_script.asm);
    println!("Unlocking Script: {}", unlocking_script.asm);
    println!();

    print!("Run this script? (y/n): ");
    io::stdout().flush().unwrap();
    let mut yn = String::new();
    io::stdin().read_line(&mut yn).unwrap();

    if yn.trim() == "y" || yn.trim().is_empty() {
        match Script::run(&[unlocking_script, locking_script]) {
            Ok(stack) => {
                println!("\nFinal Stack: {:?}", stack);
                if Script::validate(&stack) {
                    println!("{}", "\n✓ This is a valid script!".green());
                } else {
                    println!("\n✗ This is not a valid script.");
                    println!("");
                }
            }
            Err(e) => {
                println!("\n Script execution failed: {}", e);
            }
        }
    }
}