use anyhow::{anyhow, Result};
use ripemd::{Digest as RipemdDigest, Ripemd160};
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};
use sha2::{Digest, Sha256};
use std::fmt;


use crate::stack::Stack;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    OpDup,
    OpHash160,
    OpEqualVerify,
    OpCheckSig,
    OpEqual,
    OpVerify,
    OpDrop,
    OpSwap,
    OpPushBytes(u8),
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self> {
        match byte {
            0x76 => Ok(Opcode::OpDup),
            0xa9 => Ok(Opcode::OpHash160),
            0x88 => Ok(Opcode::OpEqualVerify),
            0xac => Ok(Opcode::OpCheckSig),
            0x87 => Ok(Opcode::OpEqual),
            0x69 => Ok(Opcode::OpVerify),
            0x75 => Ok(Opcode::OpDrop),
            0x7c => Ok(Opcode::OpSwap),
            1..=75 => Ok(Opcode::OpPushBytes(byte)),
            _ => Err(anyhow!("Unsupported opcode: 0x{:02x}", byte)),
        }
    }

    // pub fn name(&self) -> &str {
    //     match self {
    //         Opcode::OpDup => "OP_DUP",
    //         Opcode::OpHash160 => "OP_HASH160",
    //         Opcode::OpEqualVerify => "OP_EQUALVERIFY",
    //         Opcode::OpCheckSig => "OP_CHECKSIG",
    //         Opcode::OpEqual => "OP_EQUAL",
    //         Opcode::OpVerify => "OP_VERIFY",
    //         Opcode::OpDrop => "OP_DROP",
    //         Opcode::OpSwap => "OP_SWAP",
    //         Opcode::OpPushBytes(n) => return &format!("OP_PUSHBYTES_{}", n),
    //     }
    // }

    pub fn name(&self) -> String {
        match self {
            // Add .to_string() to the literals
            Opcode::OpDup => "OP_DUP".to_string(),
            Opcode::OpHash160 => "OP_HASH160".to_string(),
            Opcode::OpEqualVerify => "OP_EQUALVERIFY".to_string(),
            Opcode::OpCheckSig => "OP_CHECKSIG".to_string(),
            Opcode::OpEqual => "OP_EQUAL".to_string(),
            Opcode::OpVerify => "OP_VERIFY".to_string(),
            Opcode::OpDrop => "OP_DROP".to_string(),
            Opcode::OpSwap => "OP_SWAP".to_string(),
            
            Opcode::OpPushBytes(n) => format!("OP_PUSHBYTES_{}", n),
        }
    }

    pub fn execute(&self, stack: &mut Stack, script: &[u8], pos: &mut usize, sighash: &[u8]) -> Result<()> {
        println!("\nExecuting: {}", self.name());

        match self {
            Opcode::OpDup => self.op_dup(stack),
            Opcode::OpHash160 => self.op_hash160(stack),
            Opcode::OpEqualVerify => self.op_equalverify(stack),
            Opcode::OpCheckSig => self.op_checksig(stack, sighash),
            Opcode::OpEqual => self.op_equal(stack),
            Opcode::OpVerify => self.op_verify(stack),
            Opcode::OpDrop => self.op_drop(stack),
            Opcode::OpSwap => self.op_swap(stack),
            Opcode::OpPushBytes(n) => self.op_pushbytes(stack, script, pos, *n),
        }
    }

    fn op_dup(&self, stack: &mut Stack) -> Result<()> {
        let top = stack.peek()?.clone();
        stack.push(top);
        Ok(())
    }

    fn op_hash160(&self, stack: &mut Stack) -> Result<()> {
        let data = stack.pop()?;

        let sha256_hash = Sha256::digest(&data);

        let ripemd160_hash = Ripemd160::digest(sha256_hash);

        stack.push(ripemd160_hash.to_vec());
        Ok(())
    }

    fn op_equalverify(&self, stack: &mut Stack) -> Result<()> {
        let a = stack.pop()?;
        let b = stack.pop()?;

        println!("  Comparing:");
        println!("    a: {}", hex::encode(&a));
        println!("    b: {}", hex::encode(&b));

        if a != b {
            return Err(anyhow!("OP_EQUALVERIFY failed: values are not equal"));
        }

        println!("  ✓ Values are equal");
        Ok(())
    }

    fn op_checksig(&self, stack: &mut Stack, sighash: &[u8]) -> Result<()> {
        let pubkey_bytes = stack.pop()?;
        let sig_bytes = stack.pop()?;

        println!("  Signature: {} bytes", sig_bytes.len());
        println!("  Public Key: {} bytes", pubkey_bytes.len());

        if sig_bytes.is_empty() {
            return Err(anyhow!("Empty signature"));
        }

        let sig_without_hashtype = &sig_bytes[..sig_bytes.len() - 1];

        let signature = Signature::from_der(sig_without_hashtype)
            .map_err(|e| anyhow!("Invalid signature format: {}", e))?;

        let pubkey = PublicKey::from_slice(&pubkey_bytes)
            .map_err(|e| anyhow!("Invalid public key format: {}", e))?;

        let msg = Message::from_digest_slice(sighash)
            .map_err(|e| anyhow!("Invalid message hash: {}", e))?;

        let secp = Secp256k1::new();
        match secp.verify_ecdsa(&msg, &signature, &pubkey) {
            Ok(_) => {
                println!("  ✓ Signature verification successful");
                stack.push(vec![1]);
                Ok(())
            }
            Err(e) => {
                println!("  ✗ Signature verification failed: {}", e);
                stack.push(vec![0]);
                Ok(())
            }
        }
    }

    fn op_equal(&self, stack: &mut Stack) -> Result<()> {
        let a = stack.pop()?;
        let b = stack.pop()?;

        if a == b {
            stack.push(vec![1]);
        } else {
            stack.push(vec![0]);
        }
        Ok(())
    }

    fn op_verify(&self, stack: &mut Stack) -> Result<()> {
        let top = stack.pop()?;
        if top.is_empty() || top == vec![0] {
            return Err(anyhow!("OP_VERIFY failed: top stack value is false"));
        }
        Ok(())
    }

    fn op_drop(&self, stack: &mut Stack) -> Result<()> {
        stack.pop()?;
        Ok(())
    }

    fn op_swap(&self, stack: &mut Stack) -> Result<()> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        stack.push(a);
        stack.push(b);
        Ok(())
    }

    fn op_pushbytes(&self, stack: &mut Stack, script: &[u8], pos: &mut usize, n: u8) -> Result<()> {
        let n = n as usize;
        if *pos + n > script.len() {
            return Err(anyhow!("Script too short for PUSHBYTES_{}", n));
        }

        let data = script[*pos..*pos + n].to_vec();
        *pos += n;

        println!("  Pushing {} bytes: {}", n, hex::encode(&data));
        stack.push(data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_dup() {
        let mut stack = Stack::new();
        stack.push(vec![1, 2, 3]);

        let opcode = Opcode::OpDup;
        opcode.execute(&mut stack, &[], &mut 0, &[]).unwrap();

        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn test_op_hash160() {
        let mut stack = Stack::new();
        let data = b"hello world";
        stack.push(data.to_vec());

        let opcode = Opcode::OpHash160;
        opcode.execute(&mut stack, &[], &mut 0, &[]).unwrap();

        assert_eq!(stack.len(), 1);
        let result = stack.pop().unwrap();
        assert_eq!(result.len(), 20);
    }

    #[test]
    fn test_op_equalverify_success() {
        let mut stack = Stack::new();
        stack.push(vec![1, 2, 3]);
        stack.push(vec![1, 2, 3]);

        let opcode = Opcode::OpEqualVerify;
        assert!(opcode.execute(&mut stack, &[], &mut 0, &[]).is_ok());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_op_equalverify_failure() {
        let mut stack = Stack::new();
        stack.push(vec![1, 2, 3]);
        stack.push(vec![4, 5, 6]);

        let opcode = Opcode::OpEqualVerify;
        assert!(opcode.execute(&mut stack, &[], &mut 0, &[]).is_err());
    }
}
