# Public Key Derivation

## Flow
- Using the point arithmetic on the secp256k1 curve, we can derive a private key 
- Using the private key gotten and the generator point with Discrete logarithm problem difficulty to prevent reverse engineering of the private key from the public key, we can derive the public key

## Security note 
The function used for scalar multiplication in [projective_point](../point_arithmetic/projective_point.rs#L145-L162) uses the standard "Double-and-Add" method.

### Vulnerability: 
This is not constant time. The if k.is_odd() branch creates a timing variance. An attacker monitoring your CPU power usage or timing can deduce whether a bit in your private key is 0 or 1.

### Fix (Advanced): 
Production engines use the Montgomery Ladder approach, which performs an Addition and a Doubling at every step regardless of the bit value, ensuring the CPU takes the exact same time/power for every bit. 

**Note:** For Project 5, the simple Double-and-Add is sufficient for functional correctness, but be aware of the side-channel leak.

## Flow
EcPoint -> Private Key (Scalar) -> Math Engine (Jacobian) -> Result (Affine 64-byte) -> Output (Compressed 33-byte).
