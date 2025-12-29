use crate::keypair_deriv::keypair::KeyPair;
use crate::point_arithmetic::{JacobianPoint, EcPoint, get_generator_affine};
use primitive_types::U256;
use hex;

pub fn handle_generate (format: String, show_private: bool) {
    // Generate Keypair
    let keypair = KeyPair::generate();
    
    // Serialize public key based on format
    let pubkey_bytes = match format.as_str() {
        "compressed" => keypair.public_key.0.serialize_compressed().to_vec(),
        "uncompressed" => keypair.public_key.0.serialize_uncompressed().to_vec(),
        "x-only" => keypair.public_key.0.serialize_x_only().to_vec(),
        _ => panic!("Invalid format"),
    };
    
    // Display results
    println!("Public Key ({}): {}", format, hex::encode(&pubkey_bytes));
    
    if show_private {
        println!("⚠️  WARNING: Displaying private key (NEVER share this!)");
        println!("Private Key: {}", hex::encode(keypair.private_key.0.to_big_endian()));
    }
}

pub fn handle_derive(private_key_hex: String, format: String) {
    // 1. Parse private key from hex
    let private_key_bytes = hex::decode(private_key_hex)
        .expect("Invalid hex string");
    let private_key = U256::from_big_endian(&private_key_bytes);
    
    // 2. Derive public key: P = k * G
    let g_affine = get_generator_affine();
    let g_jacobian = JacobianPoint::from(g_affine);
    let public_point_jacobian = g_jacobian.scalar_mul(private_key);
    let public_point = EcPoint::from(public_point_jacobian);
    
    // 3. Serialize based on format
    let pubkey_bytes = match format.as_str() {
        "compressed" => public_point.serialize_compressed().to_vec(),
        "uncompressed" => public_point.serialize_uncompressed().to_vec(),
        "x-only" => public_point.serialize_x_only().to_vec(),
        _ => panic!("Invalid format"),
    };
    
    // 4. Display result
    println!("Public Key ({}): {}", format, hex::encode(&pubkey_bytes));
}

pub fn handle_info(private_key_hex: String){
    let private_key_bytes = hex::decode(private_key_hex.clone())
        .expect("Invalid hex string");
    let private_key = U256::from_big_endian(&private_key_bytes);
    
    let g_affine = get_generator_affine();
    let g_jacobian = JacobianPoint::from(g_affine);
    let public_point_jacobian = g_jacobian.scalar_mul(private_key);
    let public_point = EcPoint::from(public_point_jacobian);
    
    println!("=== Keypair Information ===");
    println!("Private Key: {}", private_key_hex);
    println!("\nPublic Key Formats:");
    println!("  Compressed (33 bytes):   {}", hex::encode(public_point.serialize_compressed()));
    println!("  Uncompressed (65 bytes): {}", hex::encode(public_point.serialize_uncompressed()));
    println!("  X-Only (32 bytes):       {}", hex::encode(public_point.serialize_x_only()));
}