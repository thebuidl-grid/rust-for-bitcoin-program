use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::ops::{Add, Sub, Mul, Neg};

lazy_static::lazy_static! {
    static ref P: BigUint = BigUint::from_bytes_be(&hex_literal::hex!(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F"
    ));
    static ref GX: BigUint = BigUint::from_bytes_be(&hex_literal::hex!(
        "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798"
    ));
    static ref GY: BigUint = BigUint::from_bytes_be(&hex_literal::hex!(
        "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8"
    ));
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldElement(BigUint);

impl FieldElement {
    pub fn new<V: Into<BigUint>>(v: V) -> Self {
        FieldElement(v.into() % &*P)
    }

    pub fn invert(&self) -> Self {
        FieldElement(self.0.modpow(&((*P.clone()) - 2u32), &*P))
    }
}

impl Add for FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: FieldElement) -> Self::Output {
        FieldElement::new(self.0 + rhs.0)
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: FieldElement) -> Self::Output {
        let p = &*P;
        let a = self.0;
        let b = rhs.0;
        FieldElement::new((p + a) - b)
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;
    fn mul(self, rhs: FieldElement) -> Self::Output {
        FieldElement::new(self.0 * rhs.0)
    }
}

impl Neg for FieldElement {
    type Output = FieldElement;
    fn neg(self) -> Self::Output {
        if self.0.is_zero() { self }
        else { FieldElement::new(&*P - self.0) }
    }
}

#[derive(Clone, Debug)]
pub struct Point {
    pub x: Option<FieldElement>,
    pub y: Option<FieldElement>,
}

impl Point {
    pub fn infinity() -> Self {
        Point { x: None, y: None }
    }

    pub fn generator() -> Self {
        Point {
            x: Some(FieldElement::new(GX.clone())),
            y: Some(FieldElement::new(GY.clone()))
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        if self.x.is_none() { return other.clone(); }
        if other.x.is_none() { return self.clone(); }

        let x1 = self.x.clone().unwrap();
        let y1 = self.y.clone().unwrap();
        let x2 = other.x.clone().unwrap();
        let y2 = other.y.clone().unwrap();

        if x1 == x2 {
            if (y1.clone() + y2.clone()).0.is_zero() { return Point::infinity(); }
            return self.double();
        }

        let s = (y2 - y1.clone()) * (x2.clone() - x1.clone()).invert();
        let x3 = s.clone() * s.clone() - x1.clone() - x2.clone();
        let y3 = s * (x1 - x3.clone()) - y1;

        Point { x: Some(x3), y: Some(y3) }
    }

    pub fn double(&self) -> Self {
        if self.x.is_none() { return Point::infinity(); }
        let x1 = self.x.clone().unwrap();
        let y1 = self.y.clone().unwrap();
        if y1.0.is_zero() { return Point::infinity(); }

        let s = (FieldElement::new(3u32) * x1.clone() * x1.clone()) *
                (FieldElement::new(2u32) * y1.clone()).invert();

        let x3 = s.clone() * s.clone() - x1.clone() - x1.clone();
        let y3 = s * (x1 - x3.clone()) - y1;

        Point { x: Some(x3), y: Some(y3) }
    }

    pub fn scalar_mul(&self, scalar: BigUint) -> Self {
        let mut result = Point::infinity();
        let mut add = self.clone();
        let mut k = scalar;

        while !k.is_zero() {
            if &k & BigUint::one() == BigUint::one() {
                result = result.add(&add);
            }
            add = add.double();
            k >>= 1u32;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_ops() {
        assert_eq!(FieldElement::new(2u32) + FieldElement::new(3u32), FieldElement::new(5u32));
        assert_eq!(FieldElement::new(5u32) - FieldElement::new(10u32), FieldElement::new(5u32));
        assert_eq!(FieldElement::new(5u32) * FieldElement::new(10u32), FieldElement::new(50u32));
        let a = FieldElement::new(9u32);
        assert_eq!(a.clone() * a.invert(), FieldElement::new(1u32));
    }

    #[test]
    fn point_mul() {
        let g = Point::generator();
        let two = g.add(&g);
        let doubled = g.scalar_mul(2u32.into());
        assert_eq!(two.x.unwrap().0, doubled.x.unwrap().0);
    }
}
