//! Jacobian Point (3d Ecpoint including flag(z) for infinity)
//!
//! This would be used for calculation due to the avoidance of the inverse / division cost
//!
//! EcPoint for logging and display , JacobianPoint for calculation
//!
//! Using Jacobian co-ordinates (X, Y, Z) to represent (X/Z^2, Y/Z^3) in EcPoint(x,y) coordinates

use crate::point_arithmetic::{
    G_X_BYTES, G_Y_BYTES, field_element::FieldElement, ec_point::EcPoint,
};
use primitive_types::U256;

/// Projective Point (X, Y, Z)
/// Represents (X/Z^2, Y/Z^3) in Affine coordinates
///
/// Y^2 = X^3 + aXZ^4 + bZ^6
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JacobianPoint {
    pub x: FieldElement,
    pub y: FieldElement,
    pub z: FieldElement,
}

impl JacobianPoint {
    /// To ensure that the point is at infinity, z should be zero
    pub(crate) fn is_infinity(&self) -> bool {
        self.z.value == U256::zero()
    }

    /// When setting to infinity, z == 0 , x == 0 , y can be anything
    ///
    /// Y^2 = X^3 + aXZ^4 + bZ^6
    ///
    /// Y^2.Z = X^3.Z + 7.Z^3
    pub(crate) fn infinity() -> Self {
        Self {
            x: FieldElement::new(U256::zero()),
            y: FieldElement::new(U256::from(1)),
            z: FieldElement::new(U256::zero()),
        }
    }

    /// Addition for jacobianPoint
    pub(crate) fn add(&self, other: &Self) -> Self {
        if self.is_infinity() {
            return *other;
        }
        if other.is_infinity() {
            return *self;
        }

        let z1_square = self.z * self.z;
        let z2_square = other.z * other.z;

        // Represent x co-ordinates scaled to a common denominator
        // u1 = X1.Z2^2 (normalized x co-ordinates)
        let u1 = self.x * z2_square;
        // u2 = X2.Z1^2 (normalized x co-ordinates)
        let u2 = other.x * z1_square;

        // Represent y co-ordinates scaled to a common denominator
        // s1 = Y1.Z2^3 (normalized y co-ordinates)
        let s1 = self.y * z2_square * other.z;
        // s2 = Y2.Z1^3 (normalized y co-ordinates)
        let s2 = other.y * z1_square * self.z;

        // Compute normalized coordinates to properly check if points are equal
        // In Jacobian coords, (X,Y,Z) and (λ²X, λ³Y, λZ) represent the same EcPoint
        // So we must compare the normalized values, not the raw coordinates

        // Check if the points are equal by comparing normalized coordinates
        // x1 == x2 (normalized)
        if u1 == u2 {
            // y1 == y2 (normalized)
            if s1 == s2 {
                // Same point - use point doubling
                return self.double();
            } else {
                // Same x but different y - this is a vertical line
                // The result is the point at infinity
                return Self::infinity();
            }
        }

        // h = u2 - u1 (change in x)
        let h = u2 - u1;
        // r = s2 - s1 (change in y)
        let r = s2 - s1;

        // x3 = r^2 - h^3 - 2.u1.h^2
        let x3 = (r * r) - (h * h * h) - (FieldElement::new(U256::from(2)) * u1 * h * h);
        // y3 = r.(u1.h^2 - x3) - s1.h^3
        let y3 = (r * ((u1 * (h * h)) - x3)) - (s1 * (h * h * h));
        // z3 = h.z1.z2
        let z3 = h * self.z * other.z;
        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    pub(crate) fn double(&self) -> Self {
        // Jacobian point doubling for a=0 (secp256k1: y² = x³ + 7)
        // Formula:
        //   S = 4*X*Y²
        //   M = 3*X²
        //   X' = M² - 2*S
        //   Y' = M*(S - X') - 8*Y⁴
        //   Z' = 2*Y*Z

        let a = self.x * self.x; // X²
        let b = self.y * self.y; // Y²
        let c = b * b; // Y⁴ 

        // S = 4*X*Y² = 2*(X + Y²)² - 2*X² - 2*Y⁴
        // This is an optimized way to compute 4*X*Y²
        // 2 * (self.x + b)^2 - 2 * a - 2 * c
        let s = (FieldElement::new(U256::from(2)) * ((self.x + b) * (self.x + b)))
            - (FieldElement::new(U256::from(2)) * (a + c));

        // M = 3*X² (since a=0 for secp256k1)
        let m = FieldElement::new(U256::from(3)) * a;

        // X' = M² - 2*S
        let x3 = (m * m) - (FieldElement::new(U256::from(2)) * s);

        // Y' = M*(S - X') - 8*Y⁴
        let y3 = m * (s - x3) - (FieldElement::new(U256::from(8)) * c);

        // Z' = 2*Y*Z
        let z3 = FieldElement::new(U256::from(2)) * self.y * self.z;

        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    /// Scalar multiplication for jacobianPoint
    ///
    /// Using double and add (binary expansion)
    ///
    /// scalar is the private key
    pub(crate) fn scalar_mul(&self, scalar: U256) -> Self {
        let mut result = Self::infinity();
        let mut current_point = self.clone();
        let mut k = scalar.clone();
        // while k > 0
        while k > U256::zero() {
            // odd number check
            if k.low_u64() & 1 == 1 {
                // if odd
                result = result.add(&current_point);
            }
            // always double the current point for the next bit position
            current_point = current_point.double();
            // shift the scalar by 1 bit to the right / divide by 2
            k = k >> 1;
        }
        result
    }
}

impl From<EcPoint> for JacobianPoint {
    /// Convert from EcPoint to Projective
    /// Formula: (x, y) -> (x, y, 1)
    /// Infinity -> (0, 1, 0)
    fn from(ep: EcPoint) -> Self {
        match ep {
            EcPoint::Infinity => Self::infinity(),
            EcPoint::Point { x, y } => Self {
                x,
                y,
                z: FieldElement::new(U256::from(1)),
            },
        }
    }
}

impl From<JacobianPoint> for EcPoint {
    /// Convert from Projective to Affine
    /// Formula: (X, Y, Z) -> (X/Z^2, Y/Z^3)
    /// If Z == 0, return Infinity
    fn from(jp: JacobianPoint) -> Self {
        // if the z is zero , simply return inifinity here
        if jp.is_infinity() {
            return EcPoint::Infinity;
        }

        // Optimization: if Z == 1, the point is already in affine form
        // This avoids expensive modular inverse computation
        if jp.z.value == U256::from(1) {
            return EcPoint::Point { x: jp.x, y: jp.y };
        }

        // Find z^2 and z^3
        let z_squared = jp.z * jp.z;
        let z_cubed = z_squared * jp.z;
        // find the inverse of both
        let z_squared_inv = z_squared.inverse();
        let z_cubed_inv = z_cubed.inverse();
        // return the point (x/z^2, y/z^3)
        EcPoint::Point {
            x: jp.x * z_squared_inv,
            y: jp.y * z_cubed_inv,
        }
    }
}

/// Helper function to get the secp256k1 generator point G in Jacobian coordinates
pub fn get_generator_jacobian() -> JacobianPoint {
    let gx = U256::from_big_endian(&G_X_BYTES);
    let gy = U256::from_big_endian(&G_Y_BYTES);
    JacobianPoint {
        x: FieldElement::new(gx),
        y: FieldElement::new(gy),
        z: FieldElement::new(U256::from(1)),
    }
}

/// Helper function to get the secp256k1 generator point G in affine(EcPoint) coordinates
pub fn get_generator_affine() -> EcPoint {
    let gx = U256::from_big_endian(&G_X_BYTES);
    let gy = U256::from_big_endian(&G_Y_BYTES);
    EcPoint::Point {
        x: FieldElement::new(gx),
        y: FieldElement::new(gy),
    }
}

#[cfg(test)]
mod jacobian_test {
    use super::*;

    #[test]
    fn test_for_scalar_mult() {
        let g = get_generator_jacobian();
        let scalar = U256::from(1);
        let pubkey = g.scalar_mul(scalar);
        let pubkey_affine = EcPoint::from(pubkey);
        assert_eq!(pubkey_affine, get_generator_affine());
    }

    // ========== Tests for scalar_mul() ==========

    #[test]
    fn test_scalar_mul_by_zero() {
        // 0 * G = O (point at infinity)
        let g = get_generator_jacobian();
        let result = g.scalar_mul(U256::zero());

        assert!(result.is_infinity());
    }

    #[test]
    fn test_scalar_mul_by_one() {
        // 1 * G = G
        let g = get_generator_jacobian();
        let result = g.scalar_mul(U256::from(1));
        let result_affine = EcPoint::from(result);

        assert_eq!(result_affine, get_generator_affine());
    }

    #[test]
    fn test_scalar_mul_by_two() {
        // 2 * G should equal G + G
        let g = get_generator_jacobian();
        let result = g.scalar_mul(U256::from(2));
        let expected = g.double();

        let result_affine = EcPoint::from(result);
        let expected_affine = EcPoint::from(expected);

        assert_eq!(result_affine, expected_affine);
    }

    #[test]
    fn test_scalar_mul_by_three() {
        // 3 * G should equal G + G + G
        let g = get_generator_jacobian();
        let result = g.scalar_mul(U256::from(3));
        let expected = g.add(&g.double());

        let result_affine = EcPoint::from(result);
        let expected_affine = EcPoint::from(expected);

        assert_eq!(result_affine, expected_affine);
    }

    #[test]
    fn test_scalar_mul_by_four() {
        // 4 * G should equal 2 * (2 * G)
        let g = get_generator_jacobian();
        let result = g.scalar_mul(U256::from(4));
        let expected = g.double().double();

        let result_affine = EcPoint::from(result);
        let expected_affine = EcPoint::from(expected);

        assert_eq!(result_affine, expected_affine);
    }

    #[test]
    fn test_scalar_mul_by_five() {
        // 5 * G = 4 * G + G
        let g = get_generator_jacobian();
        let result = g.scalar_mul(U256::from(5));
        let four_g = g.double().double();
        let expected = four_g.add(&g);

        let result_affine = EcPoint::from(result);
        let expected_affine = EcPoint::from(expected);

        assert_eq!(result_affine, expected_affine);
    }

    #[test]
    fn test_scalar_mul_by_seven() {
        // 7 * G (binary: 111) - tests multiple consecutive 1 bits
        let g = get_generator_jacobian();
        let result = g.scalar_mul(U256::from(7));

        // Manually compute: 7G = 4G + 2G + G
        let two_g = g.double();
        let four_g = two_g.double();
        let expected = four_g.add(&two_g).add(&g);

        let result_affine = EcPoint::from(result);
        let expected_affine = EcPoint::from(expected);

        assert_eq!(result_affine, expected_affine);
    }

    #[test]
    fn test_scalar_mul_by_eight() {
        // 8 * G = 2^3 * G (power of 2)
        let g = get_generator_jacobian();
        let result = g.scalar_mul(U256::from(8));
        let expected = g.double().double().double();

        let result_affine = EcPoint::from(result);
        let expected_affine = EcPoint::from(expected);

        assert_eq!(result_affine, expected_affine);
    }

    #[test]
    fn test_scalar_mul_by_fifteen() {
        // 15 * G (binary: 1111) - all 1 bits in lower nibble
        let g = get_generator_jacobian();
        let result = g.scalar_mul(U256::from(15));

        // 15 = 8 + 4 + 2 + 1
        let two_g = g.double();
        let four_g = two_g.double();
        let eight_g = four_g.double();
        let expected = eight_g.add(&four_g).add(&two_g).add(&g);

        let result_affine = EcPoint::from(result);
        let expected_affine = EcPoint::from(expected);

        assert_eq!(result_affine, expected_affine);
    }

    #[test]
    fn test_scalar_mul_by_sixteen() {
        // 16 * G = 2^4 * G
        let g = get_generator_jacobian();
        let result = g.scalar_mul(U256::from(16));
        let expected = g.double().double().double().double();

        let result_affine = EcPoint::from(result);
        let expected_affine = EcPoint::from(expected);

        assert_eq!(result_affine, expected_affine);
    }

    #[test]
    fn test_scalar_mul_by_large_scalar() {
        // Test with a larger scalar: 255 (binary: 11111111)
        let g = get_generator_jacobian();
        let result = g.scalar_mul(U256::from(255));

        // Verify it's not infinity
        assert!(!result.is_infinity());

        // Verify that 255*G + G = 256*G
        let result_plus_g = result.add(&g);
        let expected_256g = g.scalar_mul(U256::from(256));

        let result_affine = EcPoint::from(result_plus_g);
        let expected_affine = EcPoint::from(expected_256g);

        assert_eq!(result_affine, expected_affine);
    }

    #[test]
    fn test_scalar_mul_distributivity() {
        // k * (P + Q) = k*P + k*Q is NOT generally true for scalars
        // But we can test: k * P + m * P = (k + m) * P
        let g = get_generator_jacobian();
        let k = U256::from(7);
        let m = U256::from(5);

        let k_g = g.scalar_mul(k);
        let m_g = g.scalar_mul(m);
        let left = k_g.add(&m_g);

        let k_plus_m = k + m; // 12
        let right = g.scalar_mul(k_plus_m);

        let left_affine = EcPoint::from(left);
        let right_affine = EcPoint::from(right);

        assert_eq!(left_affine, right_affine);
    }

    #[test]
    fn test_scalar_mul_associativity() {
        // (k * m) * G = k * (m * G)
        let g = get_generator_jacobian();
        let k = U256::from(3);
        let m = U256::from(4);

        // Left: (3 * 4) * G = 12 * G
        let k_times_m = k * m;
        let left = g.scalar_mul(k_times_m);

        // Right: 3 * (4 * G)
        let m_g = g.scalar_mul(m);
        let right = m_g.scalar_mul(k);

        let left_affine = EcPoint::from(left);
        let right_affine = EcPoint::from(right);

        assert_eq!(left_affine, right_affine);
    }

    #[test]
    fn test_scalar_mul_with_infinity() {
        // k * O = O for any k
        let inf = JacobianPoint::infinity();
        let result = inf.scalar_mul(U256::from(12345));

        assert!(result.is_infinity());
    }

    #[test]
    fn test_scalar_mul_power_of_two_pattern() {
        // Test powers of 2: 2^n * G
        let g = get_generator_jacobian();

        // 2^0 = 1
        let result_1 = g.scalar_mul(U256::from(1));
        assert_eq!(EcPoint::from(result_1), get_generator_affine());

        // 2^1 = 2
        let result_2 = g.scalar_mul(U256::from(2));
        let expected_2 = g.double();
        assert_eq!(EcPoint::from(result_2), EcPoint::from(expected_2));

        // 2^2 = 4
        let result_4 = g.scalar_mul(U256::from(4));
        let expected_4 = g.double().double();
        assert_eq!(EcPoint::from(result_4), EcPoint::from(expected_4));

        // 2^3 = 8
        let result_8 = g.scalar_mul(U256::from(8));
        let expected_8 = g.double().double().double();
        assert_eq!(EcPoint::from(result_8), EcPoint::from(expected_8));
    }

    #[test]
    fn test_scalar_mul_odd_even_scalars() {
        let g = get_generator_jacobian();

        // Test even scalar: 10 = 2 * 5
        let result_10 = g.scalar_mul(U256::from(10));
        let five_g = g.scalar_mul(U256::from(5));
        let expected_10 = five_g.double();

        assert_eq!(EcPoint::from(result_10), EcPoint::from(expected_10));

        // Test odd scalar: 11 = 10 + 1
        let result_11 = g.scalar_mul(U256::from(11));
        let expected_11 = result_10.add(&g);

        assert_eq!(EcPoint::from(result_11), EcPoint::from(expected_11));
    }

    #[test]
    fn test_scalar_mul_binary_representation() {
        // Test scalar with specific binary pattern: 21 = 0b10101
        // This tests alternating bits
        let g = get_generator_jacobian();
        let result = g.scalar_mul(U256::from(21));

        // 21 = 16 + 4 + 1 = 2^4 + 2^2 + 2^0
        let one_g = g.clone();
        let four_g = g.double().double();
        let sixteen_g = four_g.double().double();
        let expected = sixteen_g.add(&four_g).add(&one_g);

        let result_affine = EcPoint::from(result);
        let expected_affine = EcPoint::from(expected);

        assert_eq!(result_affine, expected_affine);
    }

    #[test]
    fn test_scalar_mul_consistency_with_addition() {
        // Verify that k*G computed via scalar_mul matches repeated addition
        let g = get_generator_jacobian();
        let k = 6;

        // Using scalar_mul
        let result_scalar = g.scalar_mul(U256::from(k));

        // Using repeated addition
        let mut result_add = g.clone();
        for _ in 1..k {
            result_add = result_add.add(&g);
        }

        let scalar_affine = EcPoint::from(result_scalar);
        let add_affine = EcPoint::from(result_add);

        assert_eq!(scalar_affine, add_affine);
    }

    #[test]
    fn test_scalar_mul_large_power_of_two() {
        // Test 2^10 = 1024
        let g = get_generator_jacobian();
        let result = g.scalar_mul(U256::from(1024));

        // Verify by doubling 10 times
        let mut expected = g.clone();
        for _ in 0..10 {
            expected = expected.double();
        }

        let result_affine = EcPoint::from(result);
        let expected_affine = EcPoint::from(expected);

        assert_eq!(result_affine, expected_affine);
    }

    #[test]
    fn test_scalar_mul_subtraction_property() {
        // Test that k*G - m*G = (k-m)*G
        // Since we don't have subtraction, we test: (k-m)*G + m*G = k*G
        let g = get_generator_jacobian();
        let k = U256::from(20);
        let m = U256::from(8);

        let k_g = g.scalar_mul(k);
        let m_g = g.scalar_mul(m);
        let k_minus_m = k - m; // 12
        let k_minus_m_g = g.scalar_mul(k_minus_m);

        // Verify: (k-m)*G + m*G = k*G
        let left = k_minus_m_g.add(&m_g);

        let left_affine = EcPoint::from(left);
        let k_g_affine = EcPoint::from(k_g);

        assert_eq!(left_affine, k_g_affine);
    }

    // ========== Tests for is_infinity() ==========

    #[test]
    fn test_is_infinity_true() {
        let inf = JacobianPoint::infinity();
        assert!(inf.is_infinity());
    }

    #[test]
    fn test_is_infinity_false() {
        let g = get_generator_jacobian();
        assert!(!g.is_infinity());
    }

    #[test]
    fn test_is_infinity_zero_z() {
        // Any point with z = 0 should be infinity
        let point = JacobianPoint {
            x: FieldElement::new(U256::from(123)),
            y: FieldElement::new(U256::from(456)),
            z: FieldElement::new(U256::zero()),
        };
        assert!(point.is_infinity());
    }

    #[test]
    fn test_is_infinity_nonzero_z() {
        // Any point with z != 0 should not be infinity
        let point = JacobianPoint {
            x: FieldElement::new(U256::from(123)),
            y: FieldElement::new(U256::from(456)),
            z: FieldElement::new(U256::from(1)),
        };
        assert!(!point.is_infinity());
    }

    // ========== Tests for infinity() ==========

    #[test]
    fn test_infinity_creation() {
        let inf = JacobianPoint::infinity();
        assert_eq!(inf.x.value, U256::zero());
        assert_eq!(inf.y.value, U256::from(1));
        assert_eq!(inf.z.value, U256::zero());
    }

    #[test]
    fn test_infinity_is_infinity() {
        let inf = JacobianPoint::infinity();
        assert!(inf.is_infinity());
    }

    // ========== Tests for add() ==========

    #[test]
    fn test_add_infinity_left() {
        // O + P = P
        let inf = JacobianPoint::infinity();
        let g = get_generator_jacobian();

        let result = inf.add(&g);
        assert_eq!(result, g);
    }

    #[test]
    fn test_add_infinity_right() {
        // P + O = P
        let g = get_generator_jacobian();
        let inf = JacobianPoint::infinity();

        let result = g.add(&inf);
        assert_eq!(result, g);
    }

    #[test]
    fn test_add_infinity_both() {
        // O + O = O
        let inf1 = JacobianPoint::infinity();
        let inf2 = JacobianPoint::infinity();

        let result = inf1.add(&inf2);
        assert!(result.is_infinity());
    }

    #[test]
    fn test_add_same_point_calls_double() {
        // P + P should call double()
        let g = get_generator_jacobian();

        let result_add = g.add(&g);
        let result_double = g.double();

        assert_eq!(result_add, result_double);
    }

    #[test]
    fn test_add_different_points() {
        // G + 2G = 3G
        let g = get_generator_jacobian();
        let two_g = g.double();

        let three_g = g.add(&two_g);

        // Verify result is not infinity
        assert!(!three_g.is_infinity());

        // Verify z is not zero
        assert_ne!(three_g.z.value, U256::zero());
    }

    #[test]
    fn test_add_commutativity() {
        // P + Q = Q + P
        let g = get_generator_jacobian();
        let two_g = g.double();

        let p_plus_q = g.add(&two_g);
        let q_plus_p = two_g.add(&g);

        // Convert both to affine to compare (since Jacobian coords can differ)
        let affine_1 = EcPoint::from(p_plus_q);
        let affine_2 = EcPoint::from(q_plus_p);

        assert_eq!(affine_1, affine_2);
    }

    #[test]
    fn test_add_associativity() {
        // (P + Q) + R = P + (Q + R)
        let g = get_generator_jacobian();
        let two_g = g.double();
        let three_g = g.add(&two_g);

        let left = (g.add(&two_g)).add(&three_g);
        let right = g.add(&two_g.add(&three_g));

        // Convert to affine to compare
        let affine_left = EcPoint::from(left);
        let affine_right = EcPoint::from(right);
        assert_eq!(affine_left, affine_right);
    }

    // ========== Tests for double() ==========

    #[test]
    fn test_double_generator() {
        let g = get_generator_jacobian();
        let two_g = g.double();

        // Verify result is not infinity
        assert!(!two_g.is_infinity());

        // Verify z is not zero
        assert_ne!(two_g.z.value, U256::zero());
    }

    #[test]
    fn test_double_matches_add() {
        // 2P should equal P + P
        let g = get_generator_jacobian();

        let doubled = g.double();
        let added = g.add(&g);

        assert_eq!(doubled, added);
    }

    #[test]
    fn test_double_twice() {
        // 4G = 2(2G)
        let g = get_generator_jacobian();
        let two_g = g.double();
        let four_g = two_g.double();

        // Verify result is not infinity
        assert!(!four_g.is_infinity());

        // Also verify 4G = 2G + 2G
        let four_g_alt = two_g.add(&two_g);

        let affine_1 = EcPoint::from(four_g);
        let affine_2 = EcPoint::from(four_g_alt);

        assert_eq!(affine_1, affine_2);
    }

    #[test]
    fn test_double_distributive() {
        // 2(P + Q) = 2P + 2Q
        let g = get_generator_jacobian();
        let two_g = g.double();

        // Left: 2(G + 2G) = 2(3G) = 6G
        let three_g = g.add(&two_g);
        let six_g_left = three_g.double();

        // Right: 2G + 4G = 6G
        let four_g = two_g.double();
        let six_g_right = two_g.add(&four_g);

        let affine_left = EcPoint::from(six_g_left);
        let affine_right = EcPoint::from(six_g_right);

        assert_eq!(affine_left, affine_right);
    }

    // ========== Tests for EcPoint to JacobianPoint conversion ==========

    #[test]
    fn test_from_ecpoint_infinity() {
        let affine_inf = EcPoint::Infinity;
        let jacobian = JacobianPoint::from(affine_inf);

        assert!(jacobian.is_infinity());
        assert_eq!(jacobian.x.value, U256::zero());
        assert_eq!(jacobian.y.value, U256::from(1));
        assert_eq!(jacobian.z.value, U256::zero());
    }

    #[test]
    fn test_from_ecpoint_regular_point() {
        let affine_g = get_generator_affine();
        let jacobian = JacobianPoint::from(affine_g);

        // For affine (x, y) -> Jacobian (x, y, 1)
        if let EcPoint::Point { x, y } = affine_g {
            assert_eq!(jacobian.x, x);
            assert_eq!(jacobian.y, y);
            assert_eq!(jacobian.z.value, U256::from(1));
        } else {
            panic!("Expected Point, got Infinity");
        }
    }

    #[test]
    fn test_from_ecpoint_preserves_coordinates() {
        // Create a point with specific coordinates
        let x = FieldElement::new(U256::from(12345));
        let y = FieldElement::new(U256::from(67890));
        let affine = EcPoint::Point { x, y };

        let jacobian = JacobianPoint::from(affine);

        assert_eq!(jacobian.x, x);
        assert_eq!(jacobian.y, y);
        assert_eq!(jacobian.z.value, U256::from(1));
    }

    // ========== Tests for JacobianPoint to EcPoint conversion ==========

    #[test]
    fn test_to_ecpoint_infinity() {
        let jacobian_inf = JacobianPoint::infinity();
        let affine = EcPoint::from(jacobian_inf);

        assert_eq!(affine, EcPoint::Infinity);
    }

    #[test]
    fn test_to_ecpoint_z_equals_one() {
        // When z = 1, (X, Y, 1) -> (X, Y)
        let x = FieldElement::new(U256::from(12345));
        let y = FieldElement::new(U256::from(67890));
        let jacobian = JacobianPoint {
            x,
            y,
            z: FieldElement::new(U256::from(1)),
        };

        let affine = EcPoint::from(jacobian);

        if let EcPoint::Point { x: ax, y: ay } = affine {
            assert_eq!(ax, x);
            assert_eq!(ay, y);
        } else {
            panic!("Expected Point, got Infinity");
        }
    }

    #[test]
    fn test_to_ecpoint_generator() {
        let jacobian_g = get_generator_jacobian();
        let affine_g = EcPoint::from(jacobian_g);
        let expected_g = get_generator_affine();

        assert_eq!(affine_g, expected_g);
    }

    // ========== Round-trip conversion tests ==========

    #[test]
    fn test_roundtrip_infinity() {
        // Infinity -> Jacobian -> Affine -> Jacobian
        let original = EcPoint::Infinity;
        let jacobian = JacobianPoint::from(original);
        let back_to_affine = EcPoint::from(jacobian);

        assert_eq!(original, back_to_affine);
    }

    #[test]
    fn test_roundtrip_generator() {
        // Affine -> Jacobian -> Affine
        let original = get_generator_affine();
        let jacobian = JacobianPoint::from(original);
        let back_to_affine = EcPoint::from(jacobian);

        assert_eq!(original, back_to_affine);
    }

    #[test]
    fn test_roundtrip_after_operations() {
        // Test that operations in Jacobian space give same results as affine
        let g_affine = get_generator_affine();
        let g_jacobian = JacobianPoint::from(g_affine);

        // Double in Jacobian space
        let two_g_jacobian = g_jacobian.double();

        // Convert back to affine
        let two_g_affine = EcPoint::from(two_g_jacobian);

        // Double in affine space
        let two_g_affine_direct = g_affine.add(g_affine);

        assert_eq!(two_g_affine, two_g_affine_direct);
    }

    #[test]
    fn test_jacobian_addition_matches_affine() {
        // Verify that G + 2G in Jacobian gives same result as in affine
        let g_affine = get_generator_affine();
        let g_jacobian = JacobianPoint::from(g_affine);

        // Compute 2G in both systems
        let two_g_affine = g_affine.add(g_affine);
        let two_g_jacobian = g_jacobian.double();

        // Compute 3G in both systems
        let three_g_affine = g_affine.add(two_g_affine);
        let three_g_jacobian = g_jacobian.add(&two_g_jacobian);

        // Convert Jacobian result to affine and compare
        let three_g_from_jacobian = EcPoint::from(three_g_jacobian);

        assert_eq!(three_g_affine, three_g_from_jacobian);
    }

    #[test]
    fn test_multiple_operations_consistency() {
        // Test: 8G computed via repeated doubling
        let g = get_generator_jacobian();

        let two_g = g.double();
        let four_g = two_g.double();
        let eight_g = four_g.double();

        // Verify none are infinity
        assert!(!two_g.is_infinity());
        assert!(!four_g.is_infinity());
        assert!(!eight_g.is_infinity());

        // Verify 8G = 4G + 4G
        let eight_g_alt = four_g.add(&four_g);

        let affine_1 = EcPoint::from(eight_g);
        let affine_2 = EcPoint::from(eight_g_alt);

        assert_eq!(affine_1, affine_2);
    }
}
