use crate::hashing::hash160;
pub struct Opcodes;

impl Opcodes {
    pub fn get_opcode(hex: &str) -> String {
        match hex.to_lowercase().as_str() {
            "00" => "OP_0".to_string(),
            "51" => "OP_1".to_string(),
            "52" => "OP_2".to_string(),
            "53" => "OP_3".to_string(),
            "54" => "OP_4".to_string(),
            "55" => "OP_5".to_string(),
            "56" => "OP_6".to_string(),
            "57" => "OP_7".to_string(),
            "58" => "OP_8".to_string(),
            "59" => "OP_9".to_string(),
            "5a" => "OP_10".to_string(),
            "5b" => "OP_11".to_string(),
            "5c" => "OP_12".to_string(),
            "5d" => "OP_13".to_string(),
            "5e" => "OP_14".to_string(),
            "5f" => "OP_15".to_string(),
            "60" => "OP_16".to_string(),
            "6a" => "OP_RETURN".to_string(),
            "76" => "OP_DUP".to_string(),
            "87" => "OP_EQUAL".to_string(),
            "88" => "OP_EQUALVERIFY".to_string(),
            "ac" => "OP_CHECKSIG".to_string(),
            "ae" => "OP_CHECKMULTISIG".to_string(),
            "a9" => "OP_HASH160".to_string(),
            _ => hex.to_string(),
        }
    }

    pub fn get_hex(opcode: &str) -> Option<String> {
        match opcode {
            "OP_0" => Some("00".to_string()),
            "OP_1" => Some("51".to_string()),
            "OP_2" => Some("52".to_string()),
            "OP_3" => Some("53".to_string()),
            "OP_4" => Some("54".to_string()),
            "OP_5" => Some("55".to_string()),
            "OP_6" => Some("56".to_string()),
            "OP_7" => Some("57".to_string()),
            "OP_8" => Some("58".to_string()),
            "OP_9" => Some("59".to_string()),
            "OP_10" => Some("5a".to_string()),
            "OP_11" => Some("5b".to_string()),
            "OP_12" => Some("5c".to_string()),
            "OP_13" => Some("5d".to_string()),
            "OP_14" => Some("5e".to_string()),
            "OP_15" => Some("5f".to_string()),
            "OP_16" => Some("60".to_string()),
            "OP_RETURN" => Some("6a".to_string()),
            "OP_DUP" => Some("76".to_string()),
            "OP_EQUAL" => Some("87".to_string()),
            "OP_EQUALVERIFY" => Some("88".to_string()),
            "OP_CHECKSIG" => Some("ac".to_string()),
            "OP_CHECKMULTISIG" => Some("ae".to_string()),
            "OP_HASH160" => Some("a9".to_string()),
            _ => None,
        }
    }

    pub fn is_opcode(s: &str) -> bool {
        s.starts_with("OP_")
    }

    pub fn execute(opcode: &str, mut stack: Vec<String>) -> Result<Vec<String>, String> {
        match opcode {
            "OP_DUP" => {
                if stack.is_empty() {
                    return Err("Stack is empty, cannot duplicate".to_string());
                }
                let top = stack.last().unwrap().clone();
                stack.push(top);
                Ok(stack)
            }
            "OP_HASH160" => {
                if stack.is_empty() {
                    return Err("Stack is empty, cannot hash160".to_string());
                }
                let top = stack.pop().unwrap();
                let hashed = hash160(&top);
                stack.push(hashed);
                Ok(stack)
            }
            "OP_EQUAL" => {
                if stack.len() < 2 {
                    return Err("Not enough items on stack for OP_EQUAL".to_string());
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                if a == b {
                    stack.push("OP_TRUE".to_string());
                    Ok(stack)
                } else {
                    Err(format!("Items not equal: {} != {}", a, b))
                }
            }
            "OP_EQUALVERIFY" => {
                if stack.len() < 2 {
                    return Err("Not enough items on stack for OP_EQUALVERIFY".to_string());
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                if a == b {
                    Ok(stack)
                } else {
                    Err(format!("Items not equal: {} != {}", a, b))
                }
            }
            "OP_CHECKSIG" => {
                if stack.len() < 2 {
                    return Err("Not enough items on stack for OP_CHECKSIG".to_string());
                }
                let _pubkey = stack.pop().unwrap();
                let _signature = stack.pop().unwrap();
                stack.push("OP_TRUE".to_string());
                Ok(stack)
            }
            "OP_CHECKMULTISIG" => {
                if stack.is_empty() {
                    return Err("Stack empty for OP_CHECKMULTISIG".to_string());
                }
                let m_str = stack.pop().unwrap();
                let m = m_str.strip_prefix("OP_")
                    .and_then(|s| s.parse::<usize>().ok())
                    .ok_or("Invalid m value")?;
                
                if stack.len() < m {
                    return Err("Not enough signatures on stack".to_string());
                }
                for _ in 0..m {
                    stack.pop();
                }

                if stack.is_empty() {
                    return Err("Stack empty getting n value".to_string());
                }
                let n_str = stack.pop().unwrap();
                let n = n_str.strip_prefix("OP_")
                    .and_then(|s| s.parse::<usize>().ok())
                    .ok_or("Invalid n value")?;
                
                if stack.len() < n + 1 {
                    return Err("Not enough public keys on stack".to_string());
                }
                for _ in 0..n + 1 {
                    stack.pop();
                }

                stack.push("OP_TRUE".to_string());
                Ok(stack)
            }
            "OP_RETURN" => {
                Err("Script is invalid. (OP_RETURN always invalidates a script.)".to_string())
            }
            _ => Ok(stack),
        }
    }
}