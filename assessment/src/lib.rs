use hex::{decode, encode};

/// Decodes a hexadecimal string into a vector of bytes
///
/// # Arguments
/// * `hex_str` - A string slice containing hexadecimal characters
///
/// # Returns
/// * `Result<Vec<u8>, String>` - Success with byte vector or error message
pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    decode(hex_str).map_err(|e| format!("Failed to decode hex: {}", e))
}

/// Converts bytes from little-endian to big-endian (or vice versa)
///
/// # Arguments
/// * `bytes` - A slice of bytes to reverse
///
/// # Returns
/// * `Vec<u8>` - Reversed byte order
pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    bytes.iter().rev().copied().collect()
}

/// Converts a byte slice to a hexadecimal string
///
/// # Arguments
/// * `bytes` - A slice of bytes to encode
///
/// # Returns
/// * `String` - Hexadecimal representation
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    encode(bytes)
}

/// Converts a hexadecimal string to a vector of bytes
///
/// # Arguments
/// * `hex` - A string slice containing hexadecimal characters
///
/// # Returns
/// * `Result<Vec<u8>, hex::FromHexError>` - Success with byte vector or hex error
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    decode(hex)
}

/// Converts a u32 to little-endian byte array
///
/// # Arguments
/// * `num` - A 32-bit unsigned integer
///
/// # Returns
/// * `[u8; 4]` - Little-endian byte representation
pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    num.to_le_bytes()
}

/// Parses a string into satoshis (u64)
///
/// # Arguments
/// * `input` - String representation of satoshi amount
///
/// # Returns
/// * `Result<u64, String>` - Parsed value or error message
pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    input
        .parse::<u64>()
        .map_err(|_| "Invalid satoshi amount".to_string())
}

/// Bitcoin script types
#[derive(Debug, Clone, PartialEq)]
pub enum ScriptType {
    /// Pay-to-Public-Key-Hash (P2PKH)
    P2PKH,
    /// Pay-to-Script-Hash (P2SH)
    P2SH,
    /// Pay-to-Witness-Public-Key-Hash (P2WPKH)
    P2WPKH,
    /// Pay-to-Witness-Script-Hash (P2WSH)
    P2WSH,
    /// Pay-to-Taproot (P2TR)
    P2TR,
    /// Unknown or non-standard script
    Unknown,
}

/// Classifies a Bitcoin script based on its pattern
///
/// # Arguments
/// * `script` - A byte slice containing the script
///
/// # Returns
/// * `ScriptType` - The identified script type
pub fn classify_script(script: &[u8]) -> ScriptType {
    if script.is_empty() {
        return ScriptType::Unknown;
    }

    let len = script.len();

    // P2PKH: OP_DUP OP_HASH160 <20 bytes> OP_EQUALVERIFY OP_CHECKSIG (25 bytes)
    if len >= 3 && script[0] == 0x76 && script[1] == 0xa9 && script[2] == 0x14 {
        return ScriptType::P2PKH;
    }

    // P2SH: OP_HASH160 <20 bytes> OP_EQUAL (23 bytes)
    if len >= 3 && script[0] == 0xa9 && script[1] == 0x14 {
        return ScriptType::P2SH;
    }

    // P2WPKH: OP_0 <20 bytes> (22 bytes)
    if len >= 3 && script[0] == 0x00 && script[1] == 0x14 {
        return ScriptType::P2WPKH;
    }

    // P2WSH: OP_0 <32 bytes> (34 bytes)
    if len >= 3 && script[0] == 0x00 && script[1] == 0x20 {
        return ScriptType::P2WSH;
    }

    // P2TR: OP_1 <32 bytes> (34 bytes)
    if len >= 3 && script[0] == 0x51 && script[1] == 0x20 {
        return ScriptType::P2TR;
    }

    ScriptType::Unknown
}

/// Transaction outpoint (tuple struct for destructuring)
#[derive(Debug, Clone, PartialEq)]
pub struct Outpoint(pub String, pub u32);

impl Outpoint {
    /// Creates a new Outpoint
    pub fn new(txid: String, vout: u32) -> Self {
        Outpoint(txid, vout)
    }
}

/// Reads pushdata from a script
///
/// # Arguments
/// * `script` - A byte slice containing the script
///
/// # Returns
/// * `&[u8]` - The pushdata portion starting from index 2
pub fn read_pushdata(script: &[u8]) -> &[u8] {
    if script.len() > 2 { &script[2..] } else { &[] }
}

/// Wallet trait for balance operations
pub trait Wallet {
    /// Returns the current balance
    fn balance(&self) -> u64;
}

/// Test wallet implementation
pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        self.confirmed
    }
}

/// Applies a fee to a balance by subtracting it
///
/// # Arguments
/// * `balance` - Mutable reference to the balance
/// * `fee` - Fee amount to subtract
pub fn apply_fee(balance: &mut u64, fee: u64) {
    *balance = balance.saturating_sub(fee);
}

/// Formats a transaction ID for display
///
/// # Arguments
/// * `txid` - Transaction ID string
///
/// # Returns
/// * `String` - Formatted message with the txid
pub fn move_txid(txid: String) -> String {
    format!("txid: {}", txid)
}

/// Bitcoin opcodes
#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    /// Converts a byte to an Opcode
    ///
    /// # Arguments
    /// * `byte` - The opcode byte value
    ///
    /// # Returns
    /// * `Result<Self, String>` - The corresponding Opcode or error
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        match byte {
            0x76 => Ok(Opcode::OpDup),
            0xac => Ok(Opcode::OpChecksig),
            _ => Err(format!("Invalid opcode: 0x{:02x}", byte)),
        }
    }
}

/// Unspent Transaction Output (simplified for tests)
#[derive(Debug, Clone, PartialEq)]
pub struct UTXO {
    /// Transaction ID as bytes
    pub txid: Vec<u8>,
    /// Output index
    pub vout: u32,
    /// Amount in satoshis
    pub value: u64,
}

impl UTXO {
    /// Creates a new UTXO
    pub fn new(txid: Vec<u8>, vout: u32, value: u64) -> Self {
        UTXO { txid, vout, value }
    }
}

/// Consumes a UTXO (moves ownership and returns it)
///
/// # Arguments
/// * `utxo` - The UTXO to consume
///
/// # Returns
/// * `UTXO` - The same UTXO (demonstrating move semantics)
pub fn consume_utxo(utxo: UTXO) -> UTXO {
    // In a real implementation, this might mark the UTXO as spent
    // For now, we just demonstrate ownership transfer
    utxo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_hex_basic() {
        let result = decode_hex("deadbeef").unwrap();
        assert_eq!(result, vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_to_big_endian_basic() {
        let bytes = vec![0x01, 0x02, 0x03, 0x04];
        let reversed = to_big_endian(&bytes);
        assert_eq!(reversed, vec![0x04, 0x03, 0x02, 0x01]);
    }

    #[test]
    fn test_bytes_to_hex_basic() {
        let bytes = vec![0xde, 0xad, 0xbe, 0xef];
        assert_eq!(bytes_to_hex(&bytes), "deadbeef");
    }

    #[test]
    fn test_swap_endian_u32_basic() {
        let num = 0x12345678u32;
        let bytes = swap_endian_u32(num);
        assert_eq!(bytes, [0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn test_parse_satoshis_basic() {
        assert_eq!(parse_satoshis("100000000").unwrap(), 100_000_000);
        assert!(parse_satoshis("invalid").is_err());
    }

    #[test]
    fn test_apply_fee_basic() {
        let mut balance = 100_000u64;
        apply_fee(&mut balance, 1_000);
        assert_eq!(balance, 99_000);
    }

    #[test]
    fn test_wallet_trait_basic() {
        let wallet = TestWallet { confirmed: 50_000 };
        assert_eq!(wallet.balance(), 50_000);
    }

    #[test]
    fn test_opcode_from_byte_basic() {
        assert_eq!(Opcode::from_byte(0x76).unwrap(), Opcode::OpDup);
        assert_eq!(Opcode::from_byte(0xac).unwrap(), Opcode::OpChecksig);
    }

    #[test]
    fn test_outpoint_tuple() {
        let op = Outpoint("test".to_string(), 1);
        assert_eq!(op.0, "test");
        assert_eq!(op.1, 1);
    }
}
