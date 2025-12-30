use sha2::{Digest, Sha256};
use ripemd::{Ripemd160};

pub fn hash160(data: &str) -> String {
    let binary = hex::decode(data).expect("Invalid hex string");
    let sha256 = Sha256::digest(&binary);
    let ripemd = Ripemd160::digest(&sha256);
    hex::encode(ripemd)
}