use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use base58::ToBase58;
use hex;

// Compute SHA256 hash of input data and return as hex string 
pub fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

// Compute RIPEMD160 hash of input data and return as hex string
pub fn ripemd160(data: &[u8]) -> String {
    let mut hasher = Ripemd160::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

// Compute HASH160 (RIPEMD160 of SHA256) of input data and return as hex string
pub fn hash160(data: &[u8]) -> String {
    let sha = sha2::Sha256::digest(data);
    let mut ripemd = Ripemd160::new();
    ripemd.update(sha);
    let result = ripemd.finalize();
    hex::encode(result)
}

// Generate a new secp256k1 keypair
pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut secp256k1::rand::thread_rng());
    (secret_key, public_key)
}

// Derive Bitcoin P2PKH address from public key (compressed)
pub fn derive_address(public_key: &PublicKey) -> String {
    let compressed_pubkey = public_key.serialize();
    let hash160 = hash160(&compressed_pubkey);
    let hash160_bytes = hex::decode(&hash160).unwrap();

    // Version byte for mainnet P2PKH
    let mut address_bytes = vec![0x00];
    address_bytes.extend(&hash160_bytes);

    // Checksum: first 4 bytes of sha256(sha256(address_bytes))
    let checksum = {
        let mut hasher = Sha256::new();
        hasher.update(&address_bytes);
        let hash1 = hasher.finalize();
        let mut hasher2 = Sha256::new();
        hasher2.update(&hash1);
        let hash2 = hasher2.finalize();
        hash2[..4].to_vec()
    };

    address_bytes.extend(&checksum);
    address_bytes.to_base58()
}