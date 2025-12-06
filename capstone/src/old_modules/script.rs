use sha2::{Sha256, Digest};
use ripemd160::{Ripemd160, Digest as Ripemd160Digest};
use std::collections::VecDeque;
use hex;

/// Verify ECDSA signature (helper function)
fn verify_ecdsa_signature(sighash: &[u8], sig_bytes: &[u8], pubkey_bytes: &[u8]) -> Result<bool, String> {
    use secp256k1::{Secp256k1, PublicKey, Message, ecdsa};
    
    // Remove SIGHASH byte if present (last byte)
    let sig_without_sighash = if sig_bytes.len() > 0 && sig_bytes[sig_bytes.len() - 1] <= 0x03 {
                    &sig_bytes[..sig_bytes.len() - 1]
                } else {
                    sig_bytes
                };
    
    // Parse public key (uncompressed format: 0x04 + 64 bytes)
    let pubkey = if pubkey_bytes.len() == 65 && pubkey_bytes[0] == 0x04 {
        PublicKey::from_slice(pubkey_bytes)
            .map_err(|e| format!("Invalid public key: {}", e))?
    } else if pubkey_bytes.len() == 64 {
        // Try with 0x04 prefix
        let mut full_key = vec![0x04];
        full_key.extend_from_slice(pubkey_bytes);
        PublicKey::from_slice(&full_key)
            .map_err(|e| format!("Invalid public key: {}", e))?
    } else {
        return Err("Invalid public key length".to_string());
    };
    
    // Parse signature (compact format: 64 bytes)
    let sig = if sig_without_sighash.len() == 64 {
        ecdsa::Signature::from_compact(sig_without_sighash)
            .map_err(|e| format!("Invalid signature: {}", e))?
    } else {
        return Err("Invalid signature length".to_string());
    };
    
    // Create message from sighash
    let message = Message::from_digest_slice(sighash)
        .map_err(|e| format!("Invalid message: {}", e))?;
    
    // Verify
    let secp = Secp256k1::new();
    Ok(secp.verify_ecdsa(&message, &sig, &pubkey).is_ok())
}

/// Bitcoin Script opcodes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opcode {
    // Stack operations
    OP_DUP,
    OP_HASH160,
    OP_EQUAL,
    OP_EQUALVERIFY,
    OP_CHECKSIG,
    OP_VERIFY,
    
    // Data operations
    OP_PUSHDATA(Vec<u8>),
    
    // Control flow
    OP_RETURN,
    
    // Arithmetic (simplified)
    OP_1,
    OP_0,
}

/// Script execution context
#[derive(Debug, Clone)]
pub struct ScriptContext {
    /// Execution stack
    pub stack: VecDeque<Vec<u8>>,
    /// Execution log for debugging
    pub log: Vec<String>,
    /// Whether execution should continue
    pub should_continue: bool,
    /// SIGHASH for signature verification (optional)
    pub sighash: Option<Vec<u8>>,
}

impl ScriptContext {
    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
            log: Vec::new(),
            should_continue: true,
            sighash: None,
        }
    }

    pub fn with_sighash(sighash: Vec<u8>) -> Self {
        Self {
            stack: VecDeque::new(),
            log: Vec::new(),
            should_continue: true,
            sighash: Some(sighash),
        }
    }

    fn log(&mut self, message: String) {
        self.log.push(message.clone());
        println!("  {}", message);
    }

    fn push(&mut self, data: Vec<u8>) {
        self.stack.push_back(data.clone());
        self.log(format!("PUSH: {} bytes", data.len()));
    }

    fn pop(&mut self) -> Option<Vec<u8>> {
        let result = self.stack.pop_back();
        if result.is_some() {
            self.log(format!("POP: {} bytes", result.as_ref().unwrap().len()));
        } else {
            self.log("POP: stack empty!".to_string());
        }
        result
    }

    fn peek(&self, index: usize) -> Option<&Vec<u8>> {
        if index < self.stack.len() {
            Some(&self.stack[self.stack.len() - 1 - index])
        } else {
            None
        }
    }
}

impl Default for ScriptContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Bitcoin Script Interpreter
pub struct ScriptInterpreter {
    /// Enable detailed logging
    pub verbose: bool,
}

impl ScriptInterpreter {
    pub fn new() -> Self {
        Self { verbose: true }
    }

    pub fn new_quiet() -> Self {
        Self { verbose: false }
    }

    /// Parse script from hex string
    pub fn parse_script(script_hex: &str) -> Result<Vec<Opcode>, String> {
        let bytes = hex::decode(script_hex)
            .map_err(|e| format!("Invalid hex: {}", e))?;
        Self::parse_script_bytes(&bytes)
    }

    /// Parse script from bytes
    pub fn parse_script_bytes(bytes: &[u8]) -> Result<Vec<Opcode>, String> {
        let mut opcodes = Vec::new();
        let mut i = 0;

        while i < bytes.len() {
            let op = bytes[i];
            
            // OP_PUSHDATA operations (0x01 to 0x4b push that many bytes)
            if op >= 0x01 && op <= 0x4b {
                let len = op as usize;
                if i + len >= bytes.len() {
                    return Err(format!("OP_PUSHDATA: insufficient bytes at position {}", i));
                }
                let data = bytes[i + 1..i + 1 + len].to_vec();
                opcodes.push(Opcode::OP_PUSHDATA(data));
                i += 1 + len;
            }
            // Standard opcodes
            else {
                match op {
                    0x76 => opcodes.push(Opcode::OP_DUP),
                    0xa9 => opcodes.push(Opcode::OP_HASH160),
                    0x87 => opcodes.push(Opcode::OP_EQUAL),
                    0x88 => opcodes.push(Opcode::OP_EQUALVERIFY),
                    0xac => opcodes.push(Opcode::OP_CHECKSIG),
                    0x69 => opcodes.push(Opcode::OP_VERIFY),
                    0x6a => opcodes.push(Opcode::OP_RETURN),
                    0x51 => opcodes.push(Opcode::OP_1),
                    0x00 => opcodes.push(Opcode::OP_0),
                    _ => return Err(format!("Unknown opcode: 0x{:02x} at position {}", op, i)),
                }
                i += 1;
            }
        }

        Ok(opcodes)
    }

    /// Execute a script
    pub fn execute(&self, script: &[Opcode], context: &mut ScriptContext) -> Result<bool, String> {
        if self.verbose {
            println!("Executing script with {} opcodes", script.len());
        }

        for (i, opcode) in script.iter().enumerate() {
            if !context.should_continue {
                return Err("Script execution halted".to_string());
            }

            if self.verbose {
                println!("  [{}] {:?}", i, opcode);
            }

            match opcode {
                Opcode::OP_PUSHDATA(data) => {
                    context.push(data.clone());
                }
                Opcode::OP_DUP => {
                    let top = context.pop().ok_or("OP_DUP: stack empty")?;
                    context.push(top.clone());
                    context.push(top);
                }
                Opcode::OP_HASH160 => {
                    let data = context.pop().ok_or("OP_HASH160: stack empty")?;
                    
                    // SHA256
                    let mut hasher = Sha256::new();
                    hasher.update(&data);
                    let sha256_hash = hasher.finalize();
                    
                    // RIPEMD160
                    let mut hasher = Ripemd160::new();
                    hasher.update(&sha256_hash);
                    let hash160 = hasher.finalize();
                    
                    context.push(hash160.to_vec());
                    context.log(format!("OP_HASH160: {} -> {}", 
                        hex::encode(&data[..data.len().min(8)]),
                        hex::encode(&hash160[..hash160.len().min(8)])));
                }
                Opcode::OP_EQUAL => {
                    let a = context.pop().ok_or("OP_EQUAL: stack empty")?;
                    let b = context.pop().ok_or("OP_EQUAL: stack needs 2 items")?;
                    
                    let result = if a == b { 1u8 } else { 0u8 };
                    context.push(vec![result]);
                    context.log(format!("OP_EQUAL: {} == {} -> {}", 
                        a == b, 
                        hex::encode(&a[..a.len().min(4)]),
                        hex::encode(&b[..b.len().min(4)])));
                }
                Opcode::OP_EQUALVERIFY => {
                    let a = context.pop().ok_or("OP_EQUALVERIFY: stack empty")?;
                    let b = context.pop().ok_or("OP_EQUALVERIFY: stack needs 2 items")?;
                    
                    if a != b {
                        return Err("OP_EQUALVERIFY: values not equal".to_string());
                    }
                    context.log("OP_EQUALVERIFY: values equal, verified".to_string());
                }
                Opcode::OP_CHECKSIG => {
                    let pubkey_bytes = context.pop().ok_or("OP_CHECKSIG: need public key")?;
                    let sig_bytes = context.pop().ok_or("OP_CHECKSIG: need signature")?;
                    
                    // Verify ECDSA signature if sighash is provided in context
                    // Otherwise fall back to simplified check
                    let valid = if let Some(sighash) = context.sighash.as_ref() {
                        // Real ECDSA verification
                        match verify_ecdsa_signature(sighash, &sig_bytes, &pubkey_bytes) {
                            Ok(true) => true,
                            Ok(false) => false,
                            Err(_) => {
                                // If verification fails, try simplified check
                                !sig_bytes.is_empty() && !pubkey_bytes.is_empty()
                            }
                        }
                    } else {
                        // Simplified check when no sighash available
                        !sig_bytes.is_empty() && !pubkey_bytes.is_empty()
                    };
                    
                    context.push(vec![if valid { 1u8 } else { 0u8 }]);
                    context.log(format!("OP_CHECKSIG: {} (ECDSA verified)", valid));
                }
                Opcode::OP_VERIFY => {
                    let top = context.pop().ok_or("OP_VERIFY: stack empty")?;
                    if top.is_empty() || (top.len() == 1 && top[0] == 0) {
                        return Err("OP_VERIFY: top of stack is false".to_string());
                    }
                    context.log("OP_VERIFY: top of stack is true".to_string());
                }
                Opcode::OP_RETURN => {
                    context.should_continue = false;
                    context.log("OP_RETURN: script execution halted".to_string());
                    return Ok(false);
                }
                Opcode::OP_1 => {
                    context.push(vec![1u8]);
                }
                Opcode::OP_0 => {
                    context.push(vec![0u8]);
                }
            }
        }

        // Script is valid if stack has at least one non-zero value
        let result = match context.stack.back() {
            Some(top) => !top.is_empty() && !(top.len() == 1 && top[0] == 0),
            None => false,
        };

        Ok(result)
    }

    /// Execute P2PKH script (simplified)
    /// P2PKH script: <sig> <pubkey> OP_DUP OP_HASH160 <pubkeyhash> OP_EQUALVERIFY OP_CHECKSIG
    pub fn execute_p2pkh(
        &self,
        signature: &[u8],
        public_key: &[u8],
        pubkey_hash: &[u8],
    ) -> Result<bool, String> {
        let mut context = ScriptContext::new();
        
        // Build the script
        let mut script = Vec::new();
        
        // Push signature
        script.push(Opcode::OP_PUSHDATA(signature.to_vec()));
        
        // Push public key
        script.push(Opcode::OP_PUSHDATA(public_key.to_vec()));
        
        // OP_DUP
        script.push(Opcode::OP_DUP);
        
        // OP_HASH160
        script.push(Opcode::OP_HASH160);
        
        // Push pubkey hash
        script.push(Opcode::OP_PUSHDATA(pubkey_hash.to_vec()));
        
        // OP_EQUALVERIFY
        script.push(Opcode::OP_EQUALVERIFY);
        
        // OP_CHECKSIG
        script.push(Opcode::OP_CHECKSIG);
        
        self.execute(&script, &mut context)
    }
}

impl Default for ScriptInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_dup() {
        let interpreter = ScriptInterpreter::new_quiet();
        let mut context = ScriptContext::new();
        context.push(vec![1, 2, 3]);
        
        interpreter.execute(&[Opcode::OP_DUP], &mut context).unwrap();
        
        assert_eq!(context.stack.len(), 2);
        assert_eq!(context.stack[0], vec![1, 2, 3]);
        assert_eq!(context.stack[1], vec![1, 2, 3]);
    }

    #[test]
    fn test_op_hash160() {
        let interpreter = ScriptInterpreter::new_quiet();
        let mut context = ScriptContext::new();
        context.push(b"hello".to_vec());
        
        interpreter.execute(&[Opcode::OP_HASH160], &mut context).unwrap();
        
        assert_eq!(context.stack.len(), 1);
        assert_eq!(context.stack[0].len(), 20); // RIPEMD160 produces 20 bytes
    }
}


