use anyhow::{anyhow, Result};
use sha2::{Digest, Sha256};

use crate::interpreter::Interpreter;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub locktime: u32,
}

#[derive(Debug, Clone)]
pub struct TxInput {
    pub prev_tx: [u8; 32],
    pub prev_index: u32,
    pub script_sig: Vec<u8>,
    pub sequence: u32,
}

#[derive(Debug, Clone)]
pub struct TxOutput {
    pub value: u64,
    pub script_pubkey: Vec<u8>,
}

impl Transaction {
    pub fn validate_p2pkh(&self, input_index: usize, script_pubkey: &[u8], verbose: bool) -> Result<bool> {
        if input_index >= self.inputs.len() {
            return Err(anyhow!("Input index out of bounds"));
        }

        let input = &self.inputs[input_index];

        if is_p2pkh_script(script_pubkey) {
            if verbose {
                println!("✓ Detected P2PKH script pattern");
                println!("  Script structure: OP_DUP OP_HASH160 <pubkeyhash> OP_EQUALVERIFY OP_CHECKSIG");
            }
        } else if verbose {
            println!("⚠ Warning: Script does not match standard P2PKH pattern");
        }

        let sighash = self.compute_sighash(input_index, script_pubkey)?;

        if verbose {
            println!("\nSignature Hash (for OP_CHECKSIG):");
            println!("  {}", hex::encode(&sighash));
        }

        let mut interpreter = Interpreter::new(verbose);
        interpreter.execute_scripts(&input.script_sig, script_pubkey, &sighash)
    }

    pub fn compute_sighash(&self, input_index: usize, script_pubkey: &[u8]) -> Result<Vec<u8>> {
        let mut tx_copy = self.clone();

        for input in &mut tx_copy.inputs {
            input.script_sig = Vec::new();
        }

        tx_copy.inputs[input_index].script_sig = script_pubkey.to_vec();

        let serialized = tx_copy.serialize()?;

        let hash_type = vec![0x01, 0x00, 0x00, 0x00];
        let mut data = serialized;
        data.extend_from_slice(&hash_type);

        let hash1 = Sha256::digest(&data);
        let hash2 = Sha256::digest(hash1);

        Ok(hash2.to_vec())
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut result = Vec::new();

        result.extend_from_slice(&self.version.to_le_bytes());

        result.push(self.inputs.len() as u8);

        for input in &self.inputs {
            result.extend_from_slice(&input.prev_tx);
            result.extend_from_slice(&input.prev_index.to_le_bytes());

            result.push(input.script_sig.len() as u8);
            result.extend_from_slice(&input.script_sig);

            result.extend_from_slice(&input.sequence.to_le_bytes());
        }

        result.push(self.outputs.len() as u8);

        for output in &self.outputs {
            result.extend_from_slice(&output.value.to_le_bytes());

            result.push(output.script_pubkey.len() as u8);
            result.extend_from_slice(&output.script_pubkey);
        }

        result.extend_from_slice(&self.locktime.to_le_bytes());

        Ok(result)
    }

    pub fn from_hex(hex: &str) -> Result<Self> {
        let bytes = hex::decode(hex.trim())?;
        Self::deserialize(&bytes)
    }

    pub fn deserialize(data: &[u8]) -> Result<Self> {
        let mut pos = 0;

        if data.len() < 10 {
            return Err(anyhow!("Transaction data too short"));
        }

        let version = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;

        let input_count = data[pos] as usize;
        pos += 1;

        let mut inputs = Vec::new();
        for _ in 0..input_count {
            if pos + 36 > data.len() {
                return Err(anyhow!("Invalid transaction: truncated input"));
            }

            let mut prev_tx = [0u8; 32];
            prev_tx.copy_from_slice(&data[pos..pos + 32]);
            pos += 32;

            let prev_index = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
            pos += 4;

            let script_len = data[pos] as usize;
            pos += 1;

            if pos + script_len > data.len() {
                return Err(anyhow!("Invalid transaction: truncated script_sig"));
            }

            let script_sig = data[pos..pos + script_len].to_vec();
            pos += script_len;

            let sequence = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
            pos += 4;

            inputs.push(TxInput {
                prev_tx,
                prev_index,
                script_sig,
                sequence,
            });
        }

        let output_count = data[pos] as usize;
        pos += 1;

        let mut outputs = Vec::new();
        for _ in 0..output_count {
            if pos + 8 > data.len() {
                return Err(anyhow!("Invalid transaction: truncated output"));
            }

            let value = u64::from_le_bytes([
                data[pos],
                data[pos + 1],
                data[pos + 2],
                data[pos + 3],
                data[pos + 4],
                data[pos + 5],
                data[pos + 6],
                data[pos + 7],
            ]);
            pos += 8;

            let script_len = data[pos] as usize;
            pos += 1;

            if pos + script_len > data.len() {
                return Err(anyhow!("Invalid transaction: truncated script_pubkey"));
            }

            let script_pubkey = data[pos..pos + script_len].to_vec();
            pos += script_len;

            outputs.push(TxOutput {
                value,
                script_pubkey,
            });
        }

        if pos + 4 > data.len() {
            return Err(anyhow!("Invalid transaction: missing locktime"));
        }

        let locktime = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);

        Ok(Transaction {
            version,
            inputs,
            outputs,
            locktime,
        })
    }
}

pub fn is_p2pkh_script(script: &[u8]) -> bool {
    script.len() == 25
        && script[0] == 0x76
        && script[1] == 0xa9
        && script[2] == 0x14
        && script[23] == 0x88
        && script[24] == 0xac
}

pub fn create_p2pkh_script(pubkey_hash: &[u8]) -> Result<Vec<u8>> {
    if pubkey_hash.len() != 20 {
        return Err(anyhow!("Public key hash must be 20 bytes"));
    }

    let mut script = Vec::new();
    script.push(0x76);
    script.push(0xa9);
    script.push(0x14);
    script.extend_from_slice(pubkey_hash);
    script.push(0x88);
    script.push(0xac);

    Ok(script)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_p2pkh_script() {
        let valid_p2pkh = vec![
            0x76, 0xa9, 0x14,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            0x88, 0xac,
        ];
        assert!(is_p2pkh_script(&valid_p2pkh));

        let invalid = vec![0x76, 0xa9];
        assert!(!is_p2pkh_script(&invalid));
    }

    #[test]
    fn test_create_p2pkh_script() {
        let pubkey_hash = vec![1u8; 20];
        let script = create_p2pkh_script(&pubkey_hash).unwrap();

        assert_eq!(script.len(), 25);
        assert!(is_p2pkh_script(&script));
    }
}
