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

fn pause() {
    println!("\nPress Enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn mine_new_block(blockchain: &mut Blockchain) {
    let prev_block = if let Some(last) = blockchain.get_latest_block() {
        last.hash.clone()
    } else {
        "0000000000000000000000000000000000000000000000000000000000000000".to_string()
    };

    println!("\n📝 Enter block details:");
    println!("Merkle root (or press Enter for default): ");
    let mut merkle_root = String::new();
    io::stdin().read_line(&mut merkle_root).unwrap();
    let merkle_root = if merkle_root.trim().is_empty() {
        // Generate a simple random-like merkle root from timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("{:064x}", now)
    } else {
        merkle_root.trim().to_string()
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    let block = Block::new(
        1,
        prev_block,
        merkle_root,
        timestamp,
        "1a6a93b3".to_string(),
    );

    let mined_block = blockchain.mine_block(block);
    blockchain.add_block(mined_block);

    println!("\n✨ Block added to blockchain!");

    // CHANGE: Automatically save to JSON after mining.
    if let Err(e) = blockchain.save_to_disk("blockchain.json") {
        println!("⚠️  Warning: Could not auto-save blockchain: {}", e);
    } else {
        println!("💾 Auto-saved to blockchain.json");
    }
}

fn view_blockchain(blockchain: &Blockchain) {
    if blockchain.blocks.is_empty() {
        println!("\n📭 Blockchain is empty!");
        return;
    }

    println!("\n📚 Blockchain ({} blocks):", blockchain.blocks.len());
    println!("════════════════════════════════════════");

    for (i, block) in blockchain.blocks.iter().enumerate() {
        println!("\n🔗 Block {}", i);
        println!(" Version: {}", block.version);
        println!(" Prev Block: {}", block.prev_block);
        println!(" Merkle Root: {}", block.merkle_root);
        println!(" Timestamp: {}", block.timestamp);
        println!(" Bits: {}", block.bits);
        println!(" Nonce: {}", block.nonce);
        println!(" Hash: {}", block.hash);
    }
}

fn validate_blockchain(blockchain: &Blockchain) {
    println!("\n🔍 Validating blockchain...");

    if blockchain.is_valid() {
        println!("✅ Blockchain is valid!");
    } else {
        println!("❌ Blockchain is invalid!");
    }
}

fn run_simulation() {
    clear_screen();
    println!("Mining Simulator");
    pause();

    println!("\n1. Get Transactions");
    println!("-------------------");
    println!("transactions: 13(including coinbase)");
    pause();

    let version = "1";
    let prevblock = "0000000000000b60bc96a44724fd72daf9b92cf8ad00510b5224c6253ac40095";
    let merkleroot = "0e60651a9934e8f0decd1c5fde39309e48fca0cd1c84a21ddfde95033762d86c";
    let time = 1305200806u32;
    let bits = "1affffff";

    println!("\n2. Block");
    println!("--------");
    println!("version: {}", version);
    println!("prevblock: {}", prevblock);
    println!("merkleroot: {}", merkleroot);
    println!("time: {} (timestamp)", time);
    println!("bits: {}", bits);
    println!("nonce: ________");
    pause();

    let header = format!(
        "{}{}{}{}{}",
        to_little_endian(&format!("{:08x}", 1)),
        reverse_bytes(prevblock),
        reverse_bytes(merkleroot),
        to_little_endian(&format!("{:08x}", time)),
        reverse_bytes(bits)
    );

    println!("\nHeader: {}________", header);
    pause();

    // CHANGE: Easy target for simulation - requires only 2 leading zero bytes (very fast mining).
    let target = "000000000000ffffff0000000000000000000000000000000000000000000000"; 
    println!("\n3. Target");
    println!("---------");
    println!("{}", target);
    pause();

    let block = Block::new(1, prevblock.to_string(), merkleroot.to_string(), time, bits.to_string());
    let mut blockchain = Blockchain::new(target.to_string());
    blockchain.mine_block(block);
}

// For ANSI orange (no deps): Define constants for orange (256-color code 208).
const ORANGE: &str = "\x1b[38;5;208m";
const RESET: &str = "\x1b[0m";

fn print_bitcoin_logo() {
    let logo_lines = vec![
        "░░░░░░░░░░░░░░░▄▄█▀▀▀▀▀█▄▄░░░░░░░░░░░░░░░",
        "░░░░░░░░░░░░░▄█▀░░▄░▄░░░░▀█▄░░░░░░░░░░░░░",
        "░░░░░░░░░░░░░█░░░▀█▀▀▀▀▄░░░█░░░░░░░░░░░░░",
        "░░░░░░░░░░░░░█░░░░█▄▄▄▄▀░░░█░░░░░░░░░░░░░",
        "░░░░░░░░░░░░░█░░░░█░░░░█░░░█░░░░░░░░░░░░░",
        "░░░░░░░░░░░░░▀█▄░▀▀█▀█▀░░▄█▀░░░░░░░░░░░░░",
        "░░░░░░░░░░░░░░░▀▀█▄▄▄▄▄█▀▀░░░░░░░░░░░░░░░",
    ];

    for line in logo_lines {
        println!("{ORANGE}{line}{RESET}");
    }
    println!(); // Extra space after logo.
}

fn main( ) {
    clear_screen();

    let mut blockchain = Blockchain::new(
        "000000000000ffffff0000000000000000000000000000000000000000000000".to_string()
    );

    // target = '0000000000006a93b30000000000000000000000000000000000000000000000' //harder target

    // Try to load existing blockchain
    let _ = blockchain.load_from_disk("blockchain.json");

    loop {
        println!("\n{ORANGE}╔═══════════════════════════════════════╗{RESET}");
        println!("{ORANGE}║ Mini Bitcoin Blockchain CLI           ║{RESET}");
        println!("{ORANGE}╚═══════════════════════════════════════╝{RESET}");

        print_bitcoin_logo();

        println!("\n{ORANGE}1. Run mining simulation {RESET}");
        println!("{ORANGE}2. View blockchain{RESET}");
        println!("{ORANGE}3. Validate blockchain{RESET}");
        println!("{ORANGE}4. Load blockchain from disk{RESET}");
        println!("{ORANGE}5. Mine new block (interactive){RESET}"); 
        println!("{ORANGE}6. Exit{RESET}");
        println!("\nSelect an option: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => run_simulation(),
            "2" => view_blockchain(&blockchain),
            "3" => validate_blockchain(&blockchain),
            "4" => {
                if let Err(e) = blockchain.load_from_disk("blockchain.json") {
                    println!("Error loading: {}", e);
                }
            }
            "5" => mine_new_block(&mut blockchain), // CHANGE: Now accessible via menu.
            "6" => {
                println!("Goodbye! 👋");
                break;
            }
            _ => println!("Invalid option!"),
        }

        pause(); // CHANGE: Add pause after each action for better UX.
    }
}