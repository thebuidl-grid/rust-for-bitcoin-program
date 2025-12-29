//!## Point Arithmetic
//!### Implement Point Arithmetic for secp256k1

use hex_literal;
use primitive_types::U256;

use super::{FieldElement, multiply};

/// The weierstrass formula used here is `y^2 = x^3 + 7`
///
/// Original weierstrass formula is `y^2 = x^3 + ax + b`
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: FieldElement,
    y: FieldElement,
}
pub const A: U256 = U256::zero();
pub const B: U256 = U256([7, 0, 0, 0]);
/// Gx coordiante for Generator point
pub const G_X_BYTES: [u8;32] = hex_literal::hex!("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
/// Gy coordiante for Generator point
pub const G_Y_BYTES: [u8;32] = hex_literal::hex!("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8");

/// Represents a point P(x,y) on the elliptic curve
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcPoint {
    Infinity,
    Point { x: FieldElement, y: FieldElement },
}

impl EcPoint {
    pub(crate) fn new(x: FieldElement, y: FieldElement) -> Self {
        EcPoint::Point { x, y }
    }

    /// Checks if the point is at infinity
    pub(crate) fn is_infinity(&self) -> bool {
        match self {
            EcPoint::Infinity => true,
            _ => false,
        }
    }

    /// This is to change the Public key to the normal recognized
    /// standard secp256k1 format (33-byte compressed)
    pub fn serialize_compressed(&self) -> [u8;33] {
        match self {
            Self::Infinity => {
                return [0u8;33];
            }
            Self::Point { x, y } => {
                let mut res = [0u8;33];
                // 1. Determine the Prefix
                // We check the parity of the Y coordinate (is it even or odd?)
                if y.value.low_u64() & 1 == 1 {
                    res[0] = 0x03;// odd tag
                } else {
                    res[0] = 0x02;// even tag
                }
                //return the x bytes with the prefix / parity field
                let x_bytes = x.value.to_big_endian();
                res[1..33].copy_from_slice(&x_bytes);
                return res;
            }
        }
    }

    /// This is to change the Public key to the normal recognized
    /// standard secp256k1 format (65-byte uncompressed)
    ///
    /// Serializes to the "Legacy" uncompressed format (65 bytes)
    /// Format: 0x04 + X + Y
    pub fn serialize_uncompressed(&self) -> [u8;65] {
        let mut res = [0u8;65];
        match self {
            Self::Infinity => {
                return [0u8;65];
            }
            Self::Point { x, y } => {
                // The uncompressed tag
                res[0] = 0x04;
                res[1..33].copy_from_slice(&x.value.to_big_endian());
                res[33..].copy_from_slice(&y.value.to_big_endian());
                return res;
            }
        }
    }

    /// Used for Bitcoin Taproot (BIP340)
    ///
    /// Serializes to the "X-only" format (32 bytes)
    /// Format: X
    pub fn serialize_x_only(&self) -> [u8;32] {
        match self {
            Self::Infinity => {
                return [0u8;32];
            }
            Self::Point { x, y: _y } => {
                return x.value.to_big_endian();
            }
        }
    }

    pub(crate) fn add(self, other: Self) -> Self {
        match (self, other) {
            (EcPoint::Infinity, _) => other,
            (_, EcPoint::Infinity) => self,
            (EcPoint::Point { x: x1, y: y1 }, EcPoint::Point { x: x2, y: y2 }) => {
                // Case 1 if x1 == x2
                if x1 == x2 {
                    // Case 1a
                    // This means it has the same value of x but different values of y
                    // This means it is a vertical line and does not intersect at any point
                    if y1 != y2 {
                        return EcPoint::Infinity;
                    }
                    // Case 1b (y1 or y2 == 0)
                    // This means it is a vertical line and intersects at only one point
                    // Tangent is zero
                    if y1.value == U256::zero() || y2.value == U256::zero() {
                        return EcPoint::Infinity;
                    }
                    // Case 1c (y1 == y2)
                    // Point doubling
                    // If it has the same x and y for the 2 points
                    // P + P = 2P
                    // s(slope / differentiaton) = (3x^2 + a)/ 2y
                    let numerator = FieldElement::new(
                        multiply(U256::from(3), multiply(x1.value, x1.value)) + A,
                    );
                    let denominator = FieldElement::new(multiply(U256::from(2), y1.value));
                    //@note: This is where the division occurs, we try to avoid this here
                    let s = numerator / denominator;
                    let s_squared = FieldElement::new(multiply(s.value, s.value));
                    let x3 = s_squared - x1 - x2;
                    let y3 = FieldElement::new(multiply(s.value, (x1 - x3).value)) - y1;
                    return EcPoint::Point { x: x3, y: y3 };
                } else {
                    // Case 2 (x1 != x2)
                    // Point Addition (P + Q where P!=Q)
                    // s = (y2-y1)/(x2-x1)
                    //@note: This is where the division occurs, we try to avoid this here
                    let s = (y2 - y1) / (x2 - x1);
                    let s_squared = FieldElement::new(multiply(s.value, s.value));
                    let x3 = s_squared - x1 - x2;
                    let y3 = FieldElement::new(multiply(s.value, (x1 - x3).value)) - y1;
                    return EcPoint::Point { x: x3, y: y3 };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::P;
    use super::*;

    /// Helper function to verify a point is on the curve: y^2 = x^3 + 7 (mod P)
    fn is_on_curve(point: &EcPoint) -> bool {
        match point {
            EcPoint::Infinity => true,
            EcPoint::Point { x, y } => {
                let y_squared = multiply(y.value, y.value);
                let x_cubed = multiply(multiply(x.value, x.value), x.value);
                let right_side = FieldElement::new(x_cubed + B);
                y_squared == right_side.value
            }
        }
    }

    /// Helper to get the secp256k1 generator point G
    fn get_generator() -> EcPoint {
        let gx = U256::from_big_endian(&G_X_BYTES);
        let gy = U256::from_big_endian(&G_Y_BYTES);

        EcPoint::Point {
            x: FieldElement::new(gx),
            y: FieldElement::new(gy),
        }
    }

    #[test]
    fn test_identity_property_infinity_left() {
        // Test: O + P = P (where O is the point at infinity)
        let g = get_generator();

        let result = EcPoint::Infinity.add(g);
        assert_eq!(result, g);
    }

    #[test]
    fn test_identity_property_infinity_right() {
        // Test: P + O = P (where O is the point at infinity)
        let g = get_generator();

        let result = g.add(EcPoint::Infinity);
        assert_eq!(result, g);
    }

    #[test]
    fn test_commutativity() {
        // Test: P + Q = Q + P
        // Using G and 2G as our test points (both are on the curve)
        let g = get_generator();
        let two_g = g.add(g);

        let p_plus_q = g.add(two_g);
        let q_plus_p = two_g.add(g);

        assert_eq!(p_plus_q, q_plus_p);
    }

    #[test]
    fn test_associativity() {
        // Test: (P + Q) + R = P + (Q + R)
        // Using G, 2G, and 3G as our test points
        let g = get_generator();
        let two_g = g.add(g);
        let three_g = g.add(two_g);

        let left = g.add(two_g).add(three_g);
        let right = g.add(two_g.add(three_g));

        assert_eq!(left, right);
    }

    #[test]
    fn test_point_doubling() {
        // Test: P + P = 2P (point doubling)
        let g = get_generator();

        let doubled = g.add(g);

        // Verify the result is not infinity
        assert_ne!(doubled, EcPoint::Infinity);

        // Verify the doubled point is on the curve
        assert!(is_on_curve(&doubled));
    }

    #[test]
    fn test_point_doubling_at_zero_y() {
        // Test: P + P = O when y = 0 (tangent is vertical)
        // We need to find a point where y = 0
        // For y^2 = x^3 + 7, when y = 0: 0 = x^3 + 7 (mod P)
        // This means x^3 = -7 (mod P) = P - 7

        // For this test, we'll create a point with y = 0
        // Note: This point may not actually be on the curve
        let x = FieldElement::new(U256::from(5));
        let y = FieldElement::new(U256::zero());
        let p = EcPoint::Point{ x, y };

        let result = p.add(p);
        assert_eq!(result, EcPoint::Infinity);
    }

    #[test]
    fn test_inverse_property() {
        // Test: P + (-P) = O (where -P has same x but negated y)
        let g = get_generator();

        // Extract the coordinates of G
        if let EcPoint::Point { x, y } = g {
            // -G has the same x coordinate but y is negated (P - y in the field)
            let neg_y = FieldElement::new(P - y.value);
            let neg_g = EcPoint::Point { x, y: neg_y };

            let result = g.add(neg_g);
            assert_eq!(result, EcPoint::Infinity);
        } else {
            panic!("Generator should not be infinity");
        }
    }

    #[test]
    fn test_different_x_coordinates() {
        // Test point addition with different x coordinates
        // Using G and 2G which have different x coordinates
        let g = get_generator();
        let two_g = g.add(g);

        let result = g.add(two_g);

        // Result should not be infinity for different x coordinates
        assert_ne!(result, EcPoint::Infinity);

        // Verify the result is on the curve
        assert!(is_on_curve(&result));
    }

    #[test]
    fn test_secp256k1_generator_point_doubling() {
        // Test with a known point on secp256k1
        // Using the generator point G of secp256k1
        let g = get_generator();

        // Verify G is on the curve
        assert!(is_on_curve(&g));

        // Double the generator point: 2G
        let two_g = g.add(g);

        // Verify 2G is on the curve
        assert!(is_on_curve(&two_g));

        // Verify 2G is not infinity
        assert_ne!(two_g, EcPoint::Infinity);
    }

    #[test]
    fn test_secp256k1_generator_point_addition() {
        // Test G + 2G = 3G
        let g = get_generator();

        let two_g = g.add(g);
        let three_g = g.add(two_g);

        // Verify 3G is on the curve
        assert!(is_on_curve(&three_g));

        // Verify 3G is not infinity
        assert_ne!(three_g, EcPoint::Infinity);

        // Also verify that 2G + G = G + 2G (commutativity)
        let three_g_alt = two_g.add(g);
        assert_eq!(three_g, three_g_alt);
    }

    #[test]
    fn test_multiple_additions() {
        // Test: 2(P + Q) = 2P + 2Q
        // Using G and 2G
        let g = get_generator();
        let two_g = g.add(g);

        // Left side: 2(G + 2G) = 2(3G) = 6G
        let three_g = g.add(two_g);
        let six_g_left = three_g.add(three_g);

        // Right side: 2G + 4G = 6G
        let four_g = two_g.add(two_g);
        let six_g_right = two_g.add(four_g);

        assert_eq!(six_g_left, six_g_right);
    }

    #[test]
    fn test_infinity_plus_infinity() {
        // Test: O + O = O
        let result = EcPoint::Infinity.add(EcPoint::Infinity);
        assert_eq!(result, EcPoint::Infinity);
    }

    #[test]
    fn test_point_on_curve_validation() {
        // Test that our helper function correctly validates points on the curve

        // Point at infinity is always on the curve
        assert!(is_on_curve(&EcPoint::Infinity));

        // Test with the generator point (known to be on curve)
        let g = get_generator();
        assert!(is_on_curve(&g));
    }

    #[test]
    fn test_addition_preserves_curve_property() {
        // Test that adding two points on the curve results in a point on the curve
        let g = get_generator();

        // Start with G (on curve)
        assert!(is_on_curve(&g));

        // 2G should be on curve
        let two_g = g.add(g);
        assert!(is_on_curve(&two_g));

        // 3G should be on curve
        let three_g = g.add(two_g);
        assert!(is_on_curve(&three_g));

        // 4G should be on curve
        let four_g = two_g.add(two_g);
        assert!(is_on_curve(&four_g));

        // 5G should be on curve
        let five_g = two_g.add(three_g);
        assert!(is_on_curve(&five_g));
    }

    #[test]
    fn test_known_point_doubling_result() {
        // Test that 2G produces the expected result
        // This is a regression test with known values
        let g = get_generator();
        let two_g = g.add(g);

        // The expected coordinates of 2G on secp256k1
        let expected_x = U256::from_dec_str(
            "89565891926547004231252920425935692360644145829622209833684329913297188986597",
        )
        .unwrap();
        let expected_y = U256::from_dec_str(
            "12158399299693830322967808612713398636155367887041628176798871954788371653930",
        )
        .unwrap();

        let expected_two_g = EcPoint::Point {
            x: FieldElement::new(expected_x),
            y: FieldElement::new(expected_y),
        };

        assert_eq!(two_g, expected_two_g);
    }

    #[test]
    fn test_distributive_property() {
        // Test: k(P + Q) = kP + kQ for k = 2
        // This is: 2(G + 2G) = 2G + 4G
        let g = get_generator();
        let two_g = g.add(g);

        // Left side: 2(G + 2G) = 2(3G) = 6G
        let three_g = g.add(two_g);
        let left = three_g.add(three_g);

        // Right side: 2G + 4G = 6G
        let four_g = two_g.add(two_g);
        let right = two_g.add(four_g);

        assert_eq!(left, right);
    }

    #[test]
    fn test_repeated_doubling() {
        // Test: 2(2(2G)) = 8G
        let g = get_generator();

        let two_g = g.add(g);
        let four_g = two_g.add(two_g);
        let eight_g = four_g.add(four_g);

        // All should be on the curve
        assert!(is_on_curve(&two_g));
        assert!(is_on_curve(&four_g));
        assert!(is_on_curve(&eight_g));

        // None should be infinity
        assert_ne!(two_g, EcPoint::Infinity);
        assert_ne!(four_g, EcPoint::Infinity);
        assert_ne!(eight_g, EcPoint::Infinity);
    }

    // ========== Tests for serialize_compressed() ==========

    #[test]
    fn test_serialize_compressed_infinity() {
        // Infinity should serialize to all zeros
        let inf = EcPoint::Infinity;
        let serialized = inf.serialize_compressed();

        assert_eq!(serialized.len(), 33);
        assert_eq!(serialized, [0u8; 33]);
    }

    #[test]
    fn test_serialize_compressed_generator_format() {
        // Test that the generator point serializes with correct format
        let g = get_generator();
        let serialized = g.serialize_compressed();

        // Should be 33 bytes
        assert_eq!(serialized.len(), 33);

        // First byte should be either 0x02 (even y) or 0x03 (odd y)
        assert!(serialized[0] == 0x02 || serialized[0] == 0x03);

        // Remaining 32 bytes should be the x-coordinate
        if let EcPoint::Point { x, .. } = g {
            let x_bytes = x.value.to_big_endian();
            assert_eq!(&serialized[1..33], &x_bytes[..]);
        }
    }

    #[test]
    fn test_serialize_compressed_generator_parity() {
        // Test that the generator point has the correct parity prefix
        let g = get_generator();
        let serialized = g.serialize_compressed();

        if let EcPoint::Point { y, .. } = g {
            // Check if y is odd or even
            let is_y_odd = y.value.low_u64() & 1 == 1;

            if is_y_odd {
                assert_eq!(serialized[0], 0x03, "Y is odd, should have prefix 0x03");
            } else {
                assert_eq!(serialized[0], 0x02, "Y is even, should have prefix 0x02");
            }
        }
    }

    #[test]
    fn test_serialize_compressed_even_y() {
        // Create a point with even y-coordinate
        let x = FieldElement::new(U256::from(100));
        let y = FieldElement::new(U256::from(200)); // Even number
        let point = EcPoint::Point { x, y };

        let serialized = point.serialize_compressed();

        // Should have 0x02 prefix for even y
        assert_eq!(serialized[0], 0x02);

        // X-coordinate should be in bytes 1-32
        let x_bytes = x.value.to_big_endian();
        assert_eq!(&serialized[1..33], &x_bytes[..]);
    }

    #[test]
    fn test_serialize_compressed_odd_y() {
        // Create a point with odd y-coordinate
        let x = FieldElement::new(U256::from(100));
        let y = FieldElement::new(U256::from(201)); // Odd number
        let point = EcPoint::Point { x, y };

        let serialized = point.serialize_compressed();

        // Should have 0x03 prefix for odd y
        assert_eq!(serialized[0], 0x03);

        // X-coordinate should be in bytes 1-32
        let x_bytes = x.value.to_big_endian();
        assert_eq!(&serialized[1..33], &x_bytes[..]);
    }

    #[test]
    fn test_serialize_compressed_2g() {
        // Test serialization of 2G
        let g = get_generator();
        let two_g = g.add(g);

        let serialized = two_g.serialize_compressed();

        assert_eq!(serialized.len(), 33);
        assert!(serialized[0] == 0x02 || serialized[0] == 0x03);

        // Verify x-coordinate is correctly serialized
        if let EcPoint::Point { x, y } = two_g {
            let x_bytes = x.value.to_big_endian();
            assert_eq!(&serialized[1..33], &x_bytes[..]);

            // Verify parity matches
            let is_y_odd = y.value.low_u64() & 1 == 1;
            if is_y_odd {
                assert_eq!(serialized[0], 0x03);
            } else {
                assert_eq!(serialized[0], 0x02);
            }
        }
    }

    #[test]
    fn test_serialize_compressed_known_generator() {
        // Test with known secp256k1 generator point
        // The compressed form should start with 0x02 or 0x03 followed by x-coordinate
        let g = get_generator();
        let serialized = g.serialize_compressed();

        // Known x-coordinate of generator
        let expected_x_bytes = G_X_BYTES;

        assert_eq!(&serialized[1..33], &expected_x_bytes[..]);

        // Known y-coordinate to check parity
        let gy = U256::from_big_endian(&G_Y_BYTES);
        let is_y_odd = gy.low_u64() & 1 == 1;

        if is_y_odd {
            assert_eq!(serialized[0], 0x03);
        } else {
            assert_eq!(serialized[0], 0x02);
        }
    }

    // ========== Tests for serialize_uncompressed() ==========

    #[test]
    fn test_serialize_uncompressed_infinity() {
        // Infinity should serialize to all zeros
        let inf = EcPoint::Infinity;
        let serialized = inf.serialize_uncompressed();

        assert_eq!(serialized.len(), 65);
        assert_eq!(serialized, [0u8; 65]);
    }

    #[test]
    fn test_serialize_uncompressed_generator_format() {
        // Test that the generator point serializes with correct format
        let g = get_generator();
        let serialized = g.serialize_uncompressed();

        // Should be 65 bytes
        assert_eq!(serialized.len(), 65);

        // First byte should be 0x04 (uncompressed marker)
        assert_eq!(serialized[0], 0x04);

        // Bytes 1-32 should be x-coordinate, bytes 33-64 should be y-coordinate
        if let EcPoint::Point { x, y } = g {
            let x_bytes = x.value.to_big_endian();
            let y_bytes = y.value.to_big_endian();

            assert_eq!(&serialized[1..33], &x_bytes[..]);
            assert_eq!(&serialized[33..65], &y_bytes[..]);
        }
    }

    #[test]
    fn test_serialize_uncompressed_known_generator() {
        // Test with known secp256k1 generator point
        let g = get_generator();
        let serialized = g.serialize_uncompressed();

        // Should start with 0x04
        assert_eq!(serialized[0], 0x04);

        // Known coordinates of generator
        let expected_x_bytes = G_X_BYTES;
        let expected_y_bytes = G_Y_BYTES;

        assert_eq!(&serialized[1..33], &expected_x_bytes[..]);
        assert_eq!(&serialized[33..65], &expected_y_bytes[..]);
    }

    #[test]
    fn test_serialize_uncompressed_2g() {
        // Test serialization of 2G
        let g = get_generator();
        let two_g = g.add(g);

        let serialized = two_g.serialize_uncompressed();

        assert_eq!(serialized.len(), 65);
        assert_eq!(serialized[0], 0x04);

        // Verify coordinates are correctly serialized
        if let EcPoint::Point { x, y } = two_g {
            let x_bytes = x.value.to_big_endian();
            let y_bytes = y.value.to_big_endian();

            assert_eq!(&serialized[1..33], &x_bytes[..]);
            assert_eq!(&serialized[33..65], &y_bytes[..]);
        }
    }

    #[test]
    fn test_serialize_uncompressed_custom_point() {
        // Test with a custom point
        let x = FieldElement::new(U256::from(12345));
        let y = FieldElement::new(U256::from(67890));
        let point = EcPoint::Point { x, y };

        let serialized = point.serialize_uncompressed();

        assert_eq!(serialized.len(), 65);
        assert_eq!(serialized[0], 0x04);

        let x_bytes = x.value.to_big_endian();
        let y_bytes = y.value.to_big_endian();

        assert_eq!(&serialized[1..33], &x_bytes[..]);
        assert_eq!(&serialized[33..65], &y_bytes[..]);
    }

    // ========== Cross-format tests ==========

    #[test]
    fn test_compressed_and_uncompressed_same_x() {
        // Both formats should contain the same x-coordinate
        let g = get_generator();

        let compressed = g.serialize_compressed();
        let uncompressed = g.serialize_uncompressed();

        // X-coordinate should be the same in both
        assert_eq!(&compressed[1..33], &uncompressed[1..33]);
    }

    #[test]
    fn test_compressed_parity_matches_uncompressed_y() {
        // The parity bit in compressed format should match the y-coordinate in uncompressed
        let g = get_generator();

        let compressed = g.serialize_compressed();
        let uncompressed = g.serialize_uncompressed();

        // Extract y from uncompressed format
        let y_bytes = &uncompressed[33..65];
        let y = U256::from_big_endian(y_bytes);
        let is_y_odd = y.low_u64() & 1 == 1;

        // Check parity matches
        if is_y_odd {
            assert_eq!(compressed[0], 0x03);
        } else {
            assert_eq!(compressed[0], 0x02);
        }
    }

    #[test]
    fn test_serialization_consistency_multiple_points() {
        // Test that serialization is consistent across multiple points
        let g = get_generator();
        let two_g = g.add(g);
        let three_g = g.add(two_g);

        // All should serialize to 33 bytes (compressed)
        assert_eq!(g.serialize_compressed().len(), 33);
        assert_eq!(two_g.serialize_compressed().len(), 33);
        assert_eq!(three_g.serialize_compressed().len(), 33);

        // All should serialize to 65 bytes (uncompressed)
        assert_eq!(g.serialize_uncompressed().len(), 65);
        assert_eq!(two_g.serialize_uncompressed().len(), 65);
        assert_eq!(three_g.serialize_uncompressed().len(), 65);

        // All uncompressed should start with 0x04
        assert_eq!(g.serialize_uncompressed()[0], 0x04);
        assert_eq!(two_g.serialize_uncompressed()[0], 0x04);
        assert_eq!(three_g.serialize_uncompressed()[0], 0x04);
    }

    #[test]
    fn test_different_points_different_serializations() {
        // Different points should have different serializations
        let g = get_generator();
        let two_g = g.add(g);

        let g_compressed = g.serialize_compressed();
        let two_g_compressed = two_g.serialize_compressed();

        let g_uncompressed = g.serialize_uncompressed();
        let two_g_uncompressed = two_g.serialize_uncompressed();

        // Serializations should be different
        assert_ne!(g_compressed, two_g_compressed);
        assert_ne!(g_uncompressed, two_g_uncompressed);
    }

    #[test]
    fn test_serialize_large_coordinates() {
        // Test with large coordinate values (close to field prime)
        let large_x = FieldElement::new(P - U256::from(1000));
        let large_y = FieldElement::new(P - U256::from(2000));
        let point = EcPoint::Point {
            x: large_x,
            y: large_y,
        };

        let compressed = point.serialize_compressed();
        let uncompressed = point.serialize_uncompressed();

        // Verify format
        assert_eq!(compressed.len(), 33);
        assert_eq!(uncompressed.len(), 65);
        assert_eq!(uncompressed[0], 0x04);

        // Verify coordinates
        let x_bytes = large_x.value.to_big_endian();
        let y_bytes = large_y.value.to_big_endian();

        assert_eq!(&compressed[1..33], &x_bytes[..]);
        assert_eq!(&uncompressed[1..33], &x_bytes[..]);
        assert_eq!(&uncompressed[33..65], &y_bytes[..]);
    }
}
