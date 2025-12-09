use std::io::{Read, Error};
use clap::{Arg, Command};
use std::fmt;
use sha2::{Sha256, Sha512, Digest}; // https://docs.rs/sha2/latest/sha2/
use transaction::{Amount, Input, Output, Transaction, Txid};
mod transaction;


#[allow(unused_variables)]
fn read_version(transaction_hex: &str) -> u32 {
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    // println!("decode transaction in byte: {:?}", &transaction_bytes);

    // new way using READ Traits
    let mut bytes_slice = transaction_bytes.as_slice();
    let mut buffer = [0; 4];
    bytes_slice.read(&mut buffer).unwrap();   
    // number of input
    let num_inputs = transaction_bytes[4];
    println!("num inputs: {}", num_inputs);
   //  return  u32::from_le_bytes(opt_version_bytes);
    
   //  println!("Version Byte: {}", version_byte);

    // return version_byte;
     return u32::from_le_bytes(buffer);
}


fn read_u64(transaction_bytes: &mut &[u8]) -> u64 {
  let mut buffer = [0; 8];
  transaction_bytes.read(&mut buffer).unwrap();
  return u64::from_le_bytes(buffer);
}

fn read_amount(transaction_bytes: &mut &[u8]) -> Result<Amount, Error> {
  let mut buffer = [0; 8];
  transaction_bytes.read(&mut buffer)?;
  return Ok(Amount::from_sat(u64::from_le_bytes(buffer)));
}

fn read_u32(bytes_slice: &mut &[u8]) ->Result<u32, Error> {
  let mut buffer = [0; 4];
  bytes_slice.read(&mut buffer)?;
  return Ok(u32::from_le_bytes(buffer));
}

fn read_compact_size(transaction_bytes: &mut &[u8]) -> Result<u64, Error> {
     let mut compact_size = [0_u8; 1];
     transaction_bytes.read(&mut compact_size)?;
    
     // better way using rusty way with match
     match compact_size[0] {
        0..=252 => Ok(compact_size[0] as u64),
        253 => {
            let mut buffer = [0; 2];
            transaction_bytes.read(&mut buffer)?;
            Ok(u16::from_le_bytes(buffer) as u64)
        },
        254 => {
            let mut buffer = [0; 4];
            transaction_bytes.read(&mut buffer)?;
            Ok(u32::from_le_bytes(buffer) as u64)
        },
        255 => {
            let mut buffer = [0; 8];
            transaction_bytes.read(&mut buffer)?;
            Ok(u64::from_le_bytes(buffer) as u64)
        }

     }
}

fn read_txid(transaction_bytes: &mut &[u8]) -> Result<Txid, Error> {
    let mut buffer = [0; 32];
    transaction_bytes.read(&mut buffer)?;
    buffer.reverse();
    Ok(Txid::from_bytes(buffer))
}

fn read_script_size(transaction_bytes: &mut &[u8]) -> Result<String, Error> {
    let script_size = read_compact_size(transaction_bytes)? as usize;
    let mut buffer = vec![0_u8; script_size];
    transaction_bytes.read(&mut buffer[..]).unwrap();
     Ok(hex::encode(buffer)) // this retuen as String
   // Ok(buffer) // return as Vec<u8>
}

fn read_version_byte(transaction_bytes: &mut &[u8]) -> Result<u32, Error> {
    let mut buffer = [0; 4];
    // fill buffer with bytes read from transaction_bytes.
    transaction_bytes.read(&mut buffer)?;
    // takes those 4 bytes and interprets them as a little-endian 32-bit unsigned integer.
    return Ok(u32::from_le_bytes(buffer));
}

fn hash_row_transaction(row_transaction_bytes: &[u8]) -> Result<Txid, Error> {
    let mut first_hasher = Sha256::new();
    first_hasher.update(&row_transaction_bytes);
    let hash1 = first_hasher.finalize();

    let mut second_hasher = Sha256::new();
    second_hasher.update(hash1);

    let hash2 = second_hasher.finalize();

    Ok(Txid::from_bytes(hash2.into()))

}



// // 010000000001021fad2978f20da5173251fa87bb6ddecaa2255af7e7706517eb42e47b3de20bda0e00000000ffffffff4a60bd692c4cfe76e91569c5a2e29c823ea906c9eeac0d9817d3634293f3ccce0f00000000ffffffff017588e001000000001976a914133a81baeb8c8036b189e03c033ec9cbad96f94c88ac02483045022100d3341516a39dd88bb98c191f290e3e5098939af8461bd8e063ae70dd6b13597602207c72acc2487c08aeadeaab6bb43af92348588c37a50143b940839dee82a0c558012102030ef6b9a2eac63ef4d545534758ebae44e48a602939fff60cbef8ebe06eae4e02483045022100e4e96257d9d4abde83a4d9f287c5512151a7489faef12dcd5764ed3028e749e0022053c6704e9e6cd0a00209a95a9bbe3d2f9406412e862823f09f2cb8135897
// a72e012102030ef6b9a2eac63ef4d545534758ebae44e48a602939fff60cbef8ebe06eae4e00000000

pub fn decode_transaction(transaction_hex: String) -> Result<String, Box<dyn std::error::Error>> {
    // hex string into a Vec<u8> — a vector of raw bytes.
   let transaction_bytes = hex::decode(transaction_hex).map_err(|e| format!("Hex Decode Error: {}", e))?;
   // Converts the Vec<u8> into a slice reference (&[u8]).
   // A slice is a lightweight “view” into the vector’s contents — it doesn’t copy anything
   let mut bytes_slice = transaction_bytes.as_slice();

   // The Bitcoin transaction version field is 4 bytes long,
   let version = read_version_byte(&mut bytes_slice)?;
   let input_counts = read_compact_size(&mut bytes_slice)?;
   
   let mut inputs = vec![];

   for _ in 0..input_counts {
    let txid = read_txid(&mut bytes_slice)?;
    let output_index = read_version_byte(&mut bytes_slice)?;
    let script_sig = read_script_size(&mut bytes_slice)?;
    let sequence = read_version_byte(&mut bytes_slice)?;

    let input = Input {
        txid: txid, // Txid::from_bytes(txid),
        output_index: output_index,
        script_sig: script_sig.into(),
        sequence: sequence
    };

    inputs.push(input)
   }
   
   let json_inputs = serde_json::to_string_pretty(&inputs).unwrap();

   // process output
   let output_count = read_compact_size(&mut bytes_slice)?;
   let mut outputs = vec![];

   for _ in 0..output_count {
        let amount = read_amount(&mut bytes_slice)?;
        let script_pubkey = read_script_size(&mut bytes_slice)?;
        let output = Output {
            amount,
            script_pubkey: script_pubkey.into(),
        };

        outputs.push(output);
   };

   // read_u32 am passing the the transaction hex not the byte slice 
   let lock_time = read_u32(&mut bytes_slice)?;
   let transaction_id = hash_row_transaction(&transaction_bytes)?;


   let transaction = Transaction {
    transaction_id,
    version,
    inputs,
    outputs,
    lock_time,
   };
   Ok(serde_json::to_string_pretty(&transaction)?)

}

