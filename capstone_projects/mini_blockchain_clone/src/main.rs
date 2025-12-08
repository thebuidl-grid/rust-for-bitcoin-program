use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Write, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

// Block structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    version: u32,
    prev_block: String,
    merkle_root: String,
    timestamp: u32,
    bits: String,
    nonce: u32,
    hash: String,
}

impl Block {
    fn new(version: u32, prev_block: String, merkle_root: String, timestamp: u32, bits: String) -> Self {
        Block {
            version,
            prev_block,
            merkle_root,
            timestamp,
            bits,
            nonce: 0,
            hash: String::new(),
        }
    }

    fn header(&self) -> String {
        format!(
            "{}{}{}{}{}{}",
            to_little_endian(&format!("{:08x}", self.version)),
            reverse_bytes(&self.prev_block),
            reverse_bytes(&self.merkle_root),
            to_little_endian(&format!("{:08x}", self.timestamp)),
            reverse_bytes(&self.bits),
            to_little_endian(&format!("{:08x}", self.nonce))
        )
    }

    fn calculate_hash(&mut self) -> String {
        let header = self.header();
        let hash = double_sha256(&header);
        self.hash = reverse_bytes(&hash);
        self.hash.clone()
    }
}

// Blockchain structure
struct Blockchain {
    blocks: Vec<Block>,
    difficulty_target: String,
}

impl Blockchain {
    fn new(difficulty_target: String) -> Self {
        Blockchain {
            blocks: Vec::new(),
            difficulty_target,
        }
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    fn get_latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    fn mine_block(&mut self, mut block: Block) -> Block {
        println!("\n🔨 Mining block...");
        println!("Target: {}", self.difficulty_target);
        println!();

        let required_zeros = 2; 
        
        let target_prefix = "0".repeat(required_zeros * 2); // e.g., "00000000" for 4 hex zeros (2 bytes).

        let mut nonce = 0u32;
        loop {
            block.nonce = nonce;
            let hash = block.calculate_hash();

            // CHANGE: Simple leading zeros check for easy, accurate PoW validation.
            if hash.starts_with(&target_prefix) {
                println!("\n✅ Block mined successfully!");
                println!("Nonce: {}", nonce);
                println!("Hash: {}", hash);
                break;
            }

            if nonce % 10000 == 0 {
                println!("Nonce: {} | Hash: {}", nonce, &hash[..16]); // Truncate for output.
            }

            nonce += 1;

            // Safety: Prevent infinite loop if something's wrong.
            if nonce > 1_000_000 {
                println!("⚠️  Mining timeout - difficulty too high?");
                break;
            }
        }
        block
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            // Verify hash
            let mut temp_block = current.clone();
            temp_block.calculate_hash();
            if temp_block.hash != current.hash {
                println!("❌ Invalid hash at block {}", i);
                return false;
            }

            // Verify chain linkage
            if current.prev_block != previous.hash {
                println!("❌ Invalid previous block reference at block {}", i);
                return false;
            }

            // CHANGE: Use same leading zeros check for validation.
            let required_zeros = 2;
            let target_prefix = "0".repeat(required_zeros * 2);
            if !current.hash.starts_with(&target_prefix) {
                println!("❌ Block {} doesn't meet difficulty target", i);
                return false;
            }
        }
        true
    }

    fn save_to_disk(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.blocks)?;
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())?;
        println!("💾 Blockchain saved to {}", filename);
        Ok(())
    }

    fn load_from_disk(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        self.blocks = serde_json::from_reader(reader)?;
        println!("📂 Blockchain loaded from {}", filename);
        Ok(())
    }
}

// Utility functions
fn double_sha256(hex_string: &str) -> String {
    let bytes = hex::decode(hex_string).expect("Invalid hex string");
    let hash1 = Sha256::digest(&bytes);
    let hash2 = Sha256::digest(&hash1);
    hex::encode(hash2)
}

fn reverse_bytes(hex_string: &str) -> String {
    let mut chars: Vec<char> = hex_string.chars().collect();
    chars.chunks_exact_mut(2).rev().for_each(|chunk| {
        chunk.swap(0, 1);
    });
    chars.into_iter().collect()
}

fn to_little_endian(hex_string: &str) -> String {
    let padded = format!("{:0>8}", hex_string); // Pad to 8 hex chars (4 bytes).
    reverse_bytes(&padded)
}

fn main( ) {
    
}