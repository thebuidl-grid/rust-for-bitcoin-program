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
    
    // Arithmetic
    OP_1,
    OP_0,
}

impl Opcode {
    /// Parse opcode from byte
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x76 => Some(Opcode::OP_DUP),
            0xa9 => Some(Opcode::OP_HASH160),
            0x87 => Some(Opcode::OP_EQUAL),
            0x88 => Some(Opcode::OP_EQUALVERIFY),
            0xac => Some(Opcode::OP_CHECKSIG),
            0x69 => Some(Opcode::OP_VERIFY),
            0x6a => Some(Opcode::OP_RETURN),
            0x51 => Some(Opcode::OP_1),
            0x00 => Some(Opcode::OP_0),
            _ => None,
        }
    }
}

