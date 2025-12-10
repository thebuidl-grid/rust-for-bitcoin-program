use anyhow::{anyhow, Result};

use crate::opcodes::Opcode;
use crate::stack::Stack;

pub struct Interpreter {
    stack: Stack,
    verbose: bool,
}

impl Interpreter {
    pub fn new(verbose: bool) -> Self {
        Interpreter {
            stack: Stack::new(),
            verbose,
        }
    }

    pub fn execute(&mut self, script: &[u8], sighash: &[u8]) -> Result<bool> {
        if self.verbose {
            println!("\n═══════════════════════════════════════");
            println!("Starting Script Execution");
            println!("═══════════════════════════════════════");
            println!("Script length: {} bytes", script.len());
            println!("Script hex: {}", hex::encode(script));
            println!("═══════════════════════════════════════\n");
        }

        let mut pos = 0;

        while pos < script.len() {
            let byte = script[pos];
            pos += 1;

            if self.verbose {
                println!("─────────────────────────────────────");
                println!("Position: {}, Byte: 0x{:02x}", pos - 1, byte);
            }

            let opcode = Opcode::from_byte(byte)?;
            opcode.execute(&mut self.stack, script, &mut pos, sighash)?;

            if self.verbose {
                self.stack.print_state();
            }
        }

        let success = !self.stack.is_empty() && self.stack.top()?;

        if self.verbose {
            println!("\n═══════════════════════════════════════");
            println!("Script Execution Complete");
            println!("═══════════════════════════════════════");
            println!("Result: {}", if success { "✓ SUCCESS" } else { "✗ FAILURE" });
            println!("Final stack size: {}", self.stack.len());
            if !self.stack.is_empty() {
                println!("Top stack value: {}", hex::encode(self.stack.peek()?));
            }
            println!("═══════════════════════════════════════\n");
        }

        Ok(success)
    }

    pub fn execute_scripts(&mut self, script_sig: &[u8], script_pubkey: &[u8], sighash: &[u8]) -> Result<bool> {
        if self.verbose {
            println!("\n╔═══════════════════════════════════════╗");
            println!("║   BITCOIN SCRIPT VALIDATION           ║");
            println!("╚═══════════════════════════════════════╝\n");
            println!("ScriptSig:    {}", hex::encode(script_sig));
            println!("ScriptPubKey: {}", hex::encode(script_pubkey));
            println!();
        }

        if self.verbose {
            println!("┌─────────────────────────────────────┐");
            println!("│ Phase 1: Execute ScriptSig          │");
            println!("└─────────────────────────────────────┘");
        }

        let mut pos = 0;
        while pos < script_sig.len() {
            let byte = script_sig[pos];
            pos += 1;

            if self.verbose {
                println!("Position: {}, Byte: 0x{:02x}", pos - 1, byte);
            }

            let opcode = Opcode::from_byte(byte)?;
            opcode.execute(&mut self.stack, script_sig, &mut pos, sighash)?;
        }

        if self.verbose {
            println!("\n┌─────────────────────────────────────┐");
            println!("│ Phase 2: Execute ScriptPubKey       │");
            println!("└─────────────────────────────────────┘");
        }

        pos = 0;
        while pos < script_pubkey.len() {
            let byte = script_pubkey[pos];
            pos += 1;

            if self.verbose {
                println!("Position: {}, Byte: 0x{:02x}", pos - 1, byte);
            }

            let opcode = Opcode::from_byte(byte)?;
            opcode.execute(&mut self.stack, script_pubkey, &mut pos, sighash)?;
        }

        let success = !self.stack.is_empty() && self.stack.top()?;

        if self.verbose {
            println!("\n╔═══════════════════════════════════════╗");
            println!("║   VALIDATION RESULT                   ║");
            println!("╚═══════════════════════════════════════╝");
            println!("Status: {}", if success { "✓ VALID" } else { "✗ INVALID" });
            println!("Final stack size: {}", self.stack.len());
            if !self.stack.is_empty() {
                println!("Top stack value: {}", hex::encode(self.stack.peek()?));
            }
            println!();
        }

        Ok(success)
    }

    pub fn get_stack(&self) -> &Stack {
        &self.stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_script() {
        let mut interpreter = Interpreter::new(false);

        let script = vec![0x01, 0x42];
        let sighash = vec![0u8; 32];

        let result = interpreter.execute(&script, &sighash).unwrap();
        assert!(result);
    }

    #[test]
    fn test_op_dup_hash160() {
        let mut interpreter = Interpreter::new(false);

        let mut script = vec![0x13];
        script.extend_from_slice(b"hello world12345678");
        script.push(0x76);
        script.push(0xa9);

        let sighash = vec![0u8; 32];

        interpreter.execute(&script, &sighash).ok();
        assert_eq!(interpreter.get_stack().len(), 2);
    }
}
