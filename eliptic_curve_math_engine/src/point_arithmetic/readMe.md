# Point arithmetic

## File [modular_arithmetic.rs](modular_arithmetic.rs)
- Implements field arithmetic operations for secp256k1
- Includes Add, Sub, Mul, Div operations for FieldElement
### Issues encountered:
- Handling bit arrangement / overflow from type U256 during modular multiplication.

## File [point.rs](point.rs)
- Implements elliptic curve point addition
- Includes comprehensive tests for point arithmetic properties
### Features: 
- Data serialization (Public Keys, Signatures)
- 33 Bytes (32 bytes $x$ + 1 byte sign)
- Expensive (Requires 1 Inversion)
- Unique flag or specific logic

## File [JacobianPoint.rs](projective_point.rs)
- Implements projective point addition
- Includes comprehensive tests for projective point arithmetic properties
### Features: 
- Runtime Computation (Verifying, Signing)
- 96 Bytes (Three 32-byte integers)
- Cheap (12 Multiplications, 0 Inversions)
- Contains Explicit $Z=0$ for inifinity flag
### Issues encountered:
- Issues with comparison of jacobian co-ordinates and normal EcPoint co-ordinates
- Issues with conversion of Jacobian to Normal where it entered an infinite loop in the inverse - underflow caused arbitrary values in the test to enter an unwanted state - to combat this, i set it that if z is one, it shoudl just return the x and y as it is specified in the JacobianPoint


# Flow
1. Collect value in [EcPoint](point.rs)
2. Convert to [JacobianPoint](projective_point.rs)
3. Perform operations
4. Convert back to [EcPoint](point.rs)

Private Key (Scalar) -> Math Engine (Jacobian) -> Result (Affine 64-byte) -> Output (Compressed 33-byte).