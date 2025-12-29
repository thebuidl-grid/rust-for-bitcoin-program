//! Bitcoin Script Interpreter & Transaction Builder
//! 
//! A complete Rust toolkit for building, signing, and verifying Bitcoin transactions
//! using a custom script execution engine.

pub mod script;
pub mod tx;

// Re-export main types for convenience
pub use script::{ScriptInterpreter, ScriptContext, Opcode};
pub use tx::{
    TransactionBuilder, UnsignedInput, Output,
    generate_keypair, pubkey_to_hash,
    create_p2pkh_script_pubkey, create_p2pkh_script_sig,
    sign_transaction_input, create_signed_transaction,
    compute_sighash, encode_varint,
    parse_signed_transaction, extract_script_pubkey_from_output, read_varint,
};
