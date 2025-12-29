//! Deriving a keypair from points on the sec256pk1 curve
//!
//! With the wierstrass formula
use super::{private_key::PrivateKey, pubkey::PublicKey};
use primitive_types::U256;
use rand::{TryRngCore, rngs::OsRng};

use crate::point_arithmetic::{
    EcPoint, JacobianPoint, P, get_generator_affine, get_generator_jacobian,
};

/// How many points exist on the curve
///
/// CURVE_ORDER_HEX
///
/// 0 < k < N < P
pub(crate) const N: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

#[derive(Debug)]
pub struct KeyPair {
    /// scalar k
    pub private_key: PrivateKey,
    /// EcPoint (x,y)
    pub public_key: PublicKey,
}

impl KeyPair {
    pub fn generate() -> Self {
        // set the n to the N
        let n = U256::from_str_radix(N, 16).unwrap();

        // We will be accepting Affine / Ecpoint co-ordinates
        let g_affine = get_generator_affine();
        let g_jacobian = JacobianPoint::from(g_affine);
        let private_key_scalar: U256;

        // secure generation loop sampling
        loop {
            // A. Get 32 bytes of high-quality entropy from OS
            let mut bytes = [0u8; 32];
            OsRng
                .try_fill_bytes(&mut bytes)
                .expect("could not fill bytes");

            // B. Convert bytes to U256
            let temp_k = U256::from_big_endian(&bytes);

            // C. Validate: 0 < k < n
            // It is astronomically unlikely to hit 0 or >= n, but
            // strictly required for security/correctness.
            if !temp_k.is_zero() && temp_k < n {
                private_key_scalar = temp_k;
                break;
            }
        }

        // --- Step 2: Calculate Public Key ---
        // P = k * G
        // This uses the scalar multiplication engine
        // This is not very secure and is prone to private key reverse engineering as public key and Generator are known
        let public_point_jacobian = g_jacobian.scalar_mul(private_key_scalar);
        let public_point_ec = EcPoint::from(public_point_jacobian);

        // --- Step 3: Return KeyPair ---
        Self {
            private_key: PrivateKey(private_key_scalar),
            public_key: PublicKey(public_point_ec),
        }
    }

    pub fn drop<T>(self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = KeyPair::generate();
        // assert!(!keypair.private_key.0.is_zero());
        // assert!(!keypair.public_key.0.is_infinity());
        println!("KeyPair: {:#?}", keypair);
        println!("private_key: {:#?}", keypair.private_key.0);
        println!("public_key: {:#?}", keypair.public_key.0);
    }
}
