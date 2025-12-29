use rand::rngs::OsRng;

use crate::point_arithmetic::EcPoint;

#[derive(Debug)]
pub struct PublicKey(pub EcPoint);