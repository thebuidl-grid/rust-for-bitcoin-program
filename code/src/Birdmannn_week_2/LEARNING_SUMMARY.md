# Week 2 Learning Summary: Rust Fundamentals

## Overview
This week focused on building strong Rust fundamentals through three hands-on exercises. These exercises prepare for Bitcoin-focused development by covering essential Rust concepts.

---

## Task 1: Functions and Expressions

### What I Learned
- **Function Return Types**: Rust functions explicitly declare return types using `-> Type` syntax
- **Expression Blocks**: In Rust, the last expression in a function (without a semicolon) is automatically returned
- **Type Safety**: Rust's type system ensures functions work with specific data types

### Key Concepts
- Functions are defined with `fn` keyword
- Return type comes after the parameter list: `fn name(param: Type) -> ReturnType`
- Expressions (without semicolons) return values; statements (with semicolons) don't
- Floating-point arithmetic for financial calculations

### Practical Application
Created `btc_value_in_usd()` function that converts Bitcoin amounts to USD using exchange rates. This demonstrates:
- Simple mathematical operations
- Function composition
- Real-world Bitcoin use case

---

## Task 2: Control Flow & Loops

### What I Learned
- **For Loops**: Iterate over ranges using `for x in 1..=limit` syntax
- **While Loops**: Conditional iteration with mutable state management
- **If-Else Blocks**: Pattern matching for conditional logic
- **Loop Control**: Using modulo operator (`%`) for periodic actions

### Key Concepts
- `for` loops are ideal for known iteration counts
- `while` loops handle conditional iterations
- Range syntax: `1..limit` (exclusive) vs `1..=limit` (inclusive)
- Mutable variables with `mut` keyword for state tracking
- Checkpoint logic using modulo arithmetic

### Practical Application
Simulated Bitcoin block mining with:
- Basic mining loop showing block heights
- Checkpoints every 5 blocks
- Difficulty adjustment every 10 blocks
- Real-world blockchain simulation

---

## Task 3: Enums & Pattern Matching

### What I Learned
- **Enums**: Define types with multiple variants (Mainnet, Testnet, Regtest)
- **Pattern Matching**: Use `match` expressions to handle all enum variants
- **Exhaustiveness**: Rust requires handling all possible enum values
- **String References**: Return `&str` for efficient string handling

### Key Concepts
- Enum definition with `enum Name { Variant1, Variant2, ... }`
- `#[derive(Debug, Clone, Copy)]` attributes for enum functionality
- `match` expressions for comprehensive pattern matching
- Functions that accept enum references: `fn func(network: &Network)`
- Returning string slices for different network configurations

### Practical Application
Created Bitcoin network configuration system:
- Three network types: Mainnet, Testnet, Regtest
- RPC URL mapping for each network
- Network details (ports, descriptions, chain IDs)
- Demonstrates real Bitcoin development patterns

---

## Key Takeaways

### Rust Fundamentals Mastered
1. **Functions**: Clear return types and expression-based returns
2. **Control Flow**: Flexible looping with for/while and conditional logic
3. **Type System**: Enums provide type-safe alternatives to magic strings
4. **Pattern Matching**: Exhaustive matching prevents runtime errors

### Bitcoin Development Readiness
- Understanding network configurations (Mainnet/Testnet/Regtest)
- Block mining simulation concepts
- Value conversion and financial calculations
- RPC endpoint management

### Best Practices Applied
- Clear function documentation with doc comments (`///`)
- Meaningful variable names
- Proper use of Rust's type system
- Efficient string handling with references

---

## Files Created
- `task_1.rs` - Functions and Expressions (BTC to USD conversion)
- `task_2.rs` - Control Flow & Loops (Block mining simulation)
- `task_3.rs` - Enums & Pattern Matching (Network configuration)

All tasks compile successfully and demonstrate proper Rust patterns for Bitcoin development.

