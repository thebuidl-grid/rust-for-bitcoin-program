//! Bitcoin Script Interpreter
//! 
//! A complete stack-based script execution engine that validates Bitcoin scripts,
//! including P2PKH (Pay-to-Public-Key-Hash) scripts with ECDSA signature verification.

pub mod opcodes;
pub mod interpreter;
pub mod context;

pub use opcodes::Opcode;
pub use interpreter::ScriptInterpreter;
pub use context::ScriptContext;

