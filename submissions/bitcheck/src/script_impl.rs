use crate::script_type::ScriptType;
use crate::opcodes::Opcodes;

#[derive(Debug, Clone)]
pub struct Script {
    pub asm: String,
    pub hex: String,
    pub script_type: ScriptType,
}

impl Script {
    pub fn new(data: &str) -> Self {
        let (hex, asm) = if data.chars().all(|c| c.is_ascii_hexdigit() || c.is_whitespace()) 
            && !data.contains(' ') {
            let h = data.to_string();
            let a = Self::hex_to_asm(&h);
            (h, a)
        } else {
            let a = data.to_string();
            let h = Self::asm_to_hex(&a);
            (h, a)
        };

        let script_type = Self::get_type(&asm);

        Script {
            asm,
            hex,
            script_type,
        }
    }

    pub fn hex_to_asm(hex: &str) -> String {
        let mut asm = Vec::new();
        let bytes: Vec<&str> = hex.as_bytes()
            .chunks(2)
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect();
        
        let mut i = 0;
        while i < bytes.len() {
            let byte = bytes[i];
            let int_val = u8::from_str_radix(byte, 16).unwrap();

            if int_val > 0 && int_val < 0x4b {
                let data_len = int_val as usize;
                i += 1;
                let data: String = bytes[i..i + data_len].join("");
                asm.push(data);
                i += data_len;
            } else {
                asm.push(Opcodes::get_opcode(byte));
                i += 1;
            }
        }

        asm.join(" ")
    }

    pub fn asm_to_hex(asm: &str) -> String {
        let mut hex = String::new();
        let pieces: Vec<&str> = asm.split_whitespace().collect();

        for piece in pieces {
            if let Some(opcode_hex) = Opcodes::get_hex(piece) {
                hex.push_str(&opcode_hex);
            } else {
                let push = format!("{:02x}", piece.len() / 2);
                hex.push_str(&push);
                hex.push_str(piece);
            }
        }

        hex
    }

    pub fn get_type(asm: &str) -> ScriptType {
        let parts: Vec<&str> = asm.split_whitespace().collect();

        if parts.len() == 2 && parts[1] == "OP_CHECKSIG" {
            ScriptType::P2PK
        } else if parts.len() == 5 
            && parts[0] == "OP_DUP" 
            && parts[1] == "OP_HASH160" 
            && parts[3] == "OP_EQUALVERIFY" 
            && parts[4] == "OP_CHECKSIG" {
            ScriptType::P2PKH
        } else if parts.len() == 3 
            && parts[0] == "OP_HASH160" 
            && parts[2] == "OP_EQUAL" {
            ScriptType::P2SH
        } else if parts.len() >= 3
            && parts[0].starts_with("OP_")
            && parts[parts.len() - 2].starts_with("OP_")
            && parts[parts.len() - 1] == "OP_CHECKMULTISIG" {
            ScriptType::P2MS
        } else if parts.len() == 2 && parts[0] == "OP_RETURN" {
            ScriptType::Return
        } else {
            ScriptType::Unknown
        }
    }

    pub fn run(scripts: &[Script]) -> Result<Vec<String>, String> {

        if scripts.len() > 1 && scripts[1].script_type == ScriptType::P2SH {
            return Self::run_p2sh(scripts);
        }

        let mut script: Vec<String> = scripts
            .iter()
            .flat_map(|s| s.asm.split_whitespace().map(|x| x.to_string()))
            .collect();

        let mut stack: Vec<String> = Vec::new();

        while !script.is_empty() {
            let opcode = script.remove(0);
            
            if Opcodes::is_opcode(&opcode) {
                stack = Opcodes::execute(&opcode, stack)?;
            } else {
                stack.push(opcode);
            }
        }

        Ok(stack)
    }

    pub fn run_p2sh(scripts: &[Script]) -> Result<Vec<String>, String> {
        let unlocking = &scripts[0];
        let locking = &scripts[1];

        let mut stack = Self::run(&[unlocking.clone()])?;
        let stack_copy = stack.clone();

        let combined = Script::new(&format!("{}{}", unlocking.hex, locking.hex));
        stack = Self::run(&[combined])?;

        if Self::validate(&stack) {
            let mut stack_copy = stack_copy;
            let redeem_hex = stack_copy.pop().ok_or("No redeem script on stack")?;
            let redeem_script = Script::new(&redeem_hex);

            let combined2 = Script::new(&format!("{}{}", unlocking.hex, redeem_script.hex));
            let stack2 = Self::run(&[combined2])?;

            return Ok(stack2);
        }

        Ok(stack)
    }

    pub fn validate(stack: &[String]) -> bool {
        if stack.is_empty() {
            return false;
        }

        let top = &stack[stack.len() - 1];
        
        if top == "OP_TRUE" {
            return true;
        }

        if top.starts_with("OP_") {
            if let Some(num_str) = top.strip_prefix("OP_") {
                if let Ok(num) = num_str.parse::<i32>() {
                    return num > 0;
                }
            }
        }

        false
    }
}