//! Bitcoin Transaction Builder
//! 
//! A complete transaction building and signing toolkit that creates raw Bitcoin transactions,
//! computes SIGHASH, generates ECDSA signatures, and outputs raw hex format.

pub mod builder;
pub mod signing;
pub mod sighash;
pub mod parser;

pub use builder::{TransactionBuilder, UnsignedInput, Output, create_p2pkh_script_pubkey, create_p2pkh_script_sig, hex_to_le_bytes};
pub use sighash::encode_varint;
pub use signing::{generate_keypair, pubkey_to_hash, sign_transaction_input, verify_signature, create_signed_transaction};
pub use sighash::compute_sighash;
pub use parser::{parse_signed_transaction, extract_script_pubkey_from_output, read_varint};

