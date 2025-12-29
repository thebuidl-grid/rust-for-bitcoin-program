//!## Modular Arithmetic
//!### Implement Add, sub, mul and div for secp256k1 U256: ([[u64;4]]) data type

use primitive_types::{U256, U512};
use std::ops::{Add, Div, Mul, Sub};

/// Prime of the secp256k1 curve
///
/// U256: ([[u64;4]])
///
/// y^2 = x^3 + 7 mod(P)
///
/// y^2 = x^3 + ax + b mod(P)
pub const P: U256 = U256([
    0xFFFFFFFEFFFFFC2F,
    0xFFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFFF,
]);

/// FieldElement which would be the basis of our curve points
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FieldElement {
    pub value: U256,
}

impl FieldElement {
    /// Create new instance Of FieldElement type
    pub(crate) fn new(value: U256) -> Self {
        let mut res = value % P;
        if res < U256::zero() {
            res += P;
        }
        Self { value: res }
    }

    /// Using Extended Euclidean Algorithm: `ax + by = gcd(a,b)` to find inverse
    ///
    /// If gcd(a,m) == 1, then ax + my = 1, so x is the modular inverse of a mod m / Prime field
    pub(crate) fn inverse(&self) -> Self {
        // To ensure the value is not zero
        if self.value == U256::zero() {
            panic!("Cannot inverse a zero value");
        }
        // it should be from the field i.e (1 ..= P-1)
        if self.value >= P {
            panic!("the value {:?} is not in the field {:?}", self.value, P);
        }

        // Extended Euclidean Algorithm with unsigned arithmetic
        let (mut t, mut new_t) = (U256::zero(), U256::one());
        let (mut r, mut new_r) = (P, self.value);

        while new_r != U256::zero() {
            let quotient = r / new_r;

            // Update t: handle subtraction that might go negative
            // Instead of t - quotient * new_t, we compute it modulo P
            let prod = multiply(quotient, new_t);
            let next_t = if t >= prod { t - prod } else { P - (prod - t) };
            (t, new_t) = (new_t, next_t);

            // Update r: handle subtraction that might go negative
            let prod_r = multiply(quotient, new_r);
            let next_r = if r >= prod_r {
                r - prod_r
            } else {
                // This should not happen in a correct Extended Euclidean Algorithm
                // for finding modular inverse, but we handle it to prevent underflow
                P - (prod_r - r)
            };
            (r, new_r) = (new_r, next_r);
        }

        if r > U256::one() {
            panic!("the inverse does not exist");
        }

        FieldElement::new(t)
    }
}

/// Helper function to handle multiplication for U256 values and avoid overflows
///
/// This converts to a u512 which even maxU256 ^ 2 can never overflow, then performs modulo of P in u512 form,
/// Then takes the least sig bits i.e. little endian and converts back to a U256([[u64;4]])
pub(crate) fn multiply(a: U256, b: U256) -> U256 {
    let result = a.full_mul(b);
    let reduced = result % U512::from(P);
    let lower_256 = U256([reduced.0[0], reduced.0[1], reduced.0[2], reduced.0[3]]);
    lower_256
}

// Set various arithmetic for the field points
impl Add for FieldElement {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let add512: U512 = U512::from(self.value) + U512::from(other.value);
        let reduced_val = add512 % U512::from(P);
        let lower_256 = U256([
            reduced_val.0[0],
            reduced_val.0[1],
            reduced_val.0[2],
            reduced_val.0[3],
        ]);
        FieldElement::new(lower_256)
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let res = if self.value >= other.value {
            self.value - other.value
        } else {
            P - (other.value - self.value)
        };
        FieldElement::new(res)
    }
}

impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        // Use full_mul to handle 256-bit * 256-bit = 512-bit multiplication
        let result = self.value.full_mul(other.value);
        // Reduce the 512-bit result modulo P
        let reduced = result % primitive_types::U512::from(P);
        // Extract lower 256 bits from U512
        let lower_256 = U256([reduced.0[0], reduced.0[1], reduced.0[2], reduced.0[3]]);
        FieldElement::new(lower_256)
    }
}

impl Div for FieldElement {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        FieldElement::new(multiply(self.value, other.inverse().value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_element_creation() {
        let a = FieldElement::new(U256::from(5));
        assert_eq!(a.value, U256::from(5));
    }

    #[test]
    fn test_field_element_modular_reduction() {
        // Test that values >= P are reduced modulo P
        let a = FieldElement::new(P + U256::from(10));
        assert_eq!(a.value, U256::from(10));
    }

    #[test]
    fn test_add_simple() {
        let a = FieldElement::new(U256::from(5));
        let b = FieldElement::new(U256::from(7));
        let result = a + b;
        assert_eq!(result.value, U256::from(12));
    }

    #[test]
    fn test_add_with_modular_wrap() {
        // Test addition that wraps around the modulus
        let a = FieldElement::new(P - U256::from(5));
        let b = FieldElement::new(U256::from(10));
        let result = a + b;
        assert_eq!(result.value, U256::from(5));
    }

    #[test]
    fn test_sub_simple() {
        let a = FieldElement::new(U256::from(10));
        let b = FieldElement::new(U256::from(3));
        let result = a - b;
        assert_eq!(result.value, U256::from(7));
    }

    #[test]
    fn test_sub_with_modular_wrap() {
        // Test subtraction that wraps around (negative result)
        let a = FieldElement::new(U256::from(5));
        let b = FieldElement::new(U256::from(10));
        let result = a - b;
        // Result should be P - 5
        assert_eq!(result.value, P - U256::from(5));
    }

    #[test]
    fn test_mul_simple() {
        let a = FieldElement::new(U256::from(6));
        let b = FieldElement::new(U256::from(7));
        let result = a * b;
        assert_eq!(result.value, U256::from(42));
    }

    #[test]
    fn test_mul_with_modular_reduction() {
        // Test multiplication that requires modular reduction
        let a = FieldElement::new(P - U256::from(1));
        let b = FieldElement::new(U256::from(2));
        let result = a * b;
        // (P - 1) * 2 = 2P - 2 ≡ P - 2 (mod P)
        assert_eq!(result.value, P - U256::from(2));
    }

    #[test]
    fn test_inverse_simple() {
        // Test that 2 * inverse(2) ≡ 1 (mod P)
        let a = FieldElement::new(U256::from(2));
        let a_inv = a.inverse();
        let result = FieldElement::new(U256::from(2)) * a_inv;
        assert_eq!(result.value, U256::from(1));
    }

    #[test]
    fn test_inverse_larger_value() {
        // Test inverse of a larger value
        let a = FieldElement::new(U256::from(12345));
        let a_inv = a.inverse();
        let result = FieldElement::new(U256::from(12345)) * a_inv;
        assert_eq!(result.value, U256::from(1));
    }

    #[test]
    #[should_panic(expected = "Cannot inverse a zero value")]
    fn test_inverse_zero_panics() {
        let zero = FieldElement::new(U256::zero());
        zero.inverse();
    }

    #[test]
    fn test_div_simple() {
        // Test that 10 / 2 = 5
        let a = FieldElement::new(U256::from(10));
        let b = FieldElement::new(U256::from(2));
        let result = a / b;
        assert_eq!(result.value, U256::from(5));
    }

    #[test]
    fn test_div_with_inverse() {
        // Test that (a / b) * b = a
        let a = FieldElement::new(U256::from(42));
        let b = FieldElement::new(U256::from(7));
        let result = a / b;
        let check = result * FieldElement::new(U256::from(7));
        assert_eq!(check.value, U256::from(42));
    }

    #[test]
    #[should_panic(expected = "Cannot inverse a zero value")]
    fn test_div_by_zero_panics() {
        let a = FieldElement::new(U256::from(10));
        let zero = FieldElement::new(U256::zero());
        let _result = a / zero;
    }

    #[test]
    fn test_additive_identity() {
        // Test that a + 0 = a
        let a = FieldElement::new(U256::from(123));
        let zero = FieldElement::new(U256::zero());
        let result = a + zero;
        assert_eq!(result.value, U256::from(123));
    }

    #[test]
    fn test_multiplicative_identity() {
        // Test that a * 1 = a
        let a = FieldElement::new(U256::from(123));
        let one = FieldElement::new(U256::one());
        let result = a * one;
        assert_eq!(result.value, U256::from(123));
    }

    // ========== Commutativity Tests ==========

    #[test]
    fn test_addition_commutativity_simple() {
        // Test: a + b = b + a
        let a = FieldElement::new(U256::from(123));
        let b = FieldElement::new(U256::from(456));

        let left = a + b;
        let right = b + a;

        assert_eq!(left, right);
    }

    #[test]
    fn test_addition_commutativity_large_values() {
        // Test commutativity with large values near P
        let a = FieldElement::new(P - U256::from(100));
        let b = FieldElement::new(P - U256::from(200));

        let left = a + b;
        let right = b + a;

        assert_eq!(left, right);
    }

    #[test]
    fn test_addition_commutativity_with_wrap() {
        // Test commutativity when addition wraps around modulus
        let a = FieldElement::new(P - U256::from(10));
        let b = FieldElement::new(U256::from(50));

        let left = a + b;
        let right = b + a;

        assert_eq!(left, right);
    }

    #[test]
    fn test_multiplication_commutativity_simple() {
        // Test: a * b = b * a
        let a = FieldElement::new(U256::from(17));
        let b = FieldElement::new(U256::from(23));

        let left = a * b;
        let right = b * a;

        assert_eq!(left, right);
    }

    #[test]
    fn test_multiplication_commutativity_large_values() {
        // Test commutativity with large values
        let a = FieldElement::new(U256::from(123456789));
        let b = FieldElement::new(U256::from(987654321));

        let left = a * b;
        let right = b * a;

        assert_eq!(left, right);
    }

    #[test]
    fn test_multiplication_commutativity_near_modulus() {
        // Test commutativity with values near P
        let a = FieldElement::new(P - U256::from(1));
        let b = FieldElement::new(P - U256::from(2));

        let left = a * b;
        let right = b * a;

        assert_eq!(left, right);
    }

    // ========== Associativity Tests ==========

    #[test]
    fn test_addition_associativity_simple() {
        // Test: (a + b) + c = a + (b + c)
        let a = FieldElement::new(U256::from(10));
        let b = FieldElement::new(U256::from(20));
        let c = FieldElement::new(U256::from(30));

        let left = (a + b) + c;
        let right = a + (b + c);

        assert_eq!(left, right);
    }

    #[test]
    fn test_addition_associativity_large_values() {
        // Test associativity with large values
        let a = FieldElement::new(U256::from(1000000));
        let b = FieldElement::new(U256::from(2000000));
        let c = FieldElement::new(U256::from(3000000));

        let left = (a + b) + c;
        let right = a + (b + c);

        assert_eq!(left, right);
    }

    #[test]
    fn test_addition_associativity_with_wrap() {
        // Test associativity when operations wrap around modulus
        let a = FieldElement::new(P - U256::from(100));
        let b = FieldElement::new(P - U256::from(50));
        let c = FieldElement::new(U256::from(200));

        let left = (a + b) + c;
        let right = a + (b + c);

        assert_eq!(left, right);
    }

    #[test]
    fn test_multiplication_associativity_simple() {
        // Test: (a * b) * c = a * (b * c)
        let a = FieldElement::new(U256::from(5));
        let b = FieldElement::new(U256::from(7));
        let c = FieldElement::new(U256::from(11));

        let left = (a * b) * c;
        let right = a * (b * c);

        assert_eq!(left, right);
    }

    #[test]
    fn test_multiplication_associativity_large_values() {
        // Test associativity with larger values
        let a = FieldElement::new(U256::from(12345));
        let b = FieldElement::new(U256::from(67890));
        let c = FieldElement::new(U256::from(11111));

        let left = (a * b) * c;
        let right = a * (b * c);

        assert_eq!(left, right);
    }

    #[test]
    fn test_multiplication_associativity_near_modulus() {
        // Test associativity with values that cause modular reduction
        let a = FieldElement::new(P - U256::from(10));
        let b = FieldElement::new(U256::from(100));
        let c = FieldElement::new(U256::from(50));

        let left = (a * b) * c;
        let right = a * (b * c);

        assert_eq!(left, right);
    }

    // ========== Distributivity Tests ==========

    #[test]
    fn test_distributivity_simple() {
        // Test: a * (b + c) = a * b + a * c
        let a = FieldElement::new(U256::from(5));
        let b = FieldElement::new(U256::from(7));
        let c = FieldElement::new(U256::from(11));

        let left = a * (b + c);
        let right = (a * b) + (a * c);

        assert_eq!(left, right);
    }

    #[test]
    fn test_distributivity_large_values() {
        // Test distributivity with larger values
        let a = FieldElement::new(U256::from(12345));
        let b = FieldElement::new(U256::from(67890));
        let c = FieldElement::new(U256::from(11111));

        let left = a * (b + c);
        let right = (a * b) + (a * c);

        assert_eq!(left, right);
    }

    #[test]
    fn test_distributivity_with_modular_wrap() {
        // Test distributivity when operations wrap around modulus
        let a = FieldElement::new(U256::from(1000));
        let b = FieldElement::new(P - U256::from(100));
        let c = FieldElement::new(U256::from(200));

        let left = a * (b + c);
        let right = (a * b) + (a * c);

        assert_eq!(left, right);
    }

    #[test]
    fn test_distributivity_reverse() {
        // Test: (a + b) * c = a * c + b * c
        let a = FieldElement::new(U256::from(13));
        let b = FieldElement::new(U256::from(17));
        let c = FieldElement::new(U256::from(19));

        let left = (a + b) * c;
        let right = (a * c) + (b * c);

        assert_eq!(left, right);
    }

    #[test]
    fn test_distributivity_with_subtraction() {
        // Test: a * (b - c) = a * b - a * c
        let a = FieldElement::new(U256::from(5));
        let b = FieldElement::new(U256::from(20));
        let c = FieldElement::new(U256::from(8));

        let left = a * (b - c);
        let right = (a * b) - (a * c);

        assert_eq!(left, right);
    }

    #[test]
    fn test_distributivity_subtraction_with_wrap() {
        // Test distributivity with subtraction that wraps
        let a = FieldElement::new(U256::from(100));
        let b = FieldElement::new(U256::from(50));
        let c = FieldElement::new(U256::from(80));

        let left = a * (b - c);
        let right = (a * b) - (a * c);

        assert_eq!(left, right);
    }

    // ========== Combined Property Tests ==========

    #[test]
    fn test_combined_commutativity_associativity() {
        // Test: (a + b) + c = (b + a) + c = c + (a + b)
        let a = FieldElement::new(U256::from(100));
        let b = FieldElement::new(U256::from(200));
        let c = FieldElement::new(U256::from(300));

        let result1 = (a + b) + c;
        let result2 = (b + a) + c;
        let result3 = c + (a + b);

        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }

    #[test]
    fn test_combined_distributivity_commutativity() {
        // Test: a * (b + c) = a * (c + b) = (b + c) * a
        let a = FieldElement::new(U256::from(7));
        let b = FieldElement::new(U256::from(11));
        let c = FieldElement::new(U256::from(13));

        let result1 = a * (b + c);
        let result2 = a * (c + b);
        let result3 = (b + c) * a;

        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }

    #[test]
    fn test_complex_expression() {
        // Test: (a + b) * (c + d) = a*c + a*d + b*c + b*d
        let a = FieldElement::new(U256::from(5));
        let b = FieldElement::new(U256::from(7));
        let c = FieldElement::new(U256::from(11));
        let d = FieldElement::new(U256::from(13));

        let left = (a + b) * (c + d);
        let right = (a * c) + (a * d) + (b * c) + (b * d);

        assert_eq!(left, right);
    }

    #[test]
    fn test_subtraction_commutativity_absence() {
        // Verify that subtraction is NOT commutative (a - b ≠ b - a in general)
        let a = FieldElement::new(U256::from(100));
        let b = FieldElement::new(U256::from(50));

        let left = a - b;
        let right = b - a;

        // These should be different
        assert_ne!(left, right);

        // But they should sum to zero (mod P)
        let sum = left + right;
        assert_eq!(sum.value, U256::zero());
    }

    #[test]
    fn test_division_commutativity_absence() {
        // Verify that division is NOT commutative (a / b ≠ b / a in general)
        let a = FieldElement::new(U256::from(100));
        let b = FieldElement::new(U256::from(50));

        let left = a / b;
        let right = b / a;

        // These should be different
        assert_ne!(left, right);

        // But (a/b) * (b/a) should equal 1
        let product = left * right;
        assert_eq!(product.value, U256::one());
    }
}
