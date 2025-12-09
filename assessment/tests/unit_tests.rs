use rust_week_2_exercises::*;

#[test]
fn test_decode_hex_and_endianness() {
    let hex = "6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd73800100000";
    let le_bytes = decode_hex(hex).unwrap();
    let be_bytes = to_big_endian(&le_bytes);
    assert_eq!(le_bytes.len(), 32);
    assert_eq!(be_bytes, le_bytes.iter().rev().cloned().collect::<Vec<_>>());
}

#[test]
fn test_hex_conversion() {
    let bytes = vec![0x01, 0x02, 0xff];
    assert_eq!(bytes_to_hex(&bytes), "0102ff");
    assert_eq!(hex_to_bytes("0102ff").unwrap(), bytes);
}

#[test]
fn test_endianness_swap() {
    assert_eq!(swap_endian_u32(0x12345678), [0x78, 0x56, 0x34, 0x12]);
}

#[test]
fn test_parse_satoshis_errors() {
    assert_eq!(parse_satoshis("1000").unwrap(), 1000);
    assert_eq!(parse_satoshis("abc").unwrap_err(), "Invalid satoshi amount");
}

#[test]
fn test_script_classification() {
    assert!(matches!(
        classify_script(&[0x76, 0xa9, 0x14]),
        ScriptType::P2PKH
    ));
    assert!(matches!(
        classify_script(&[0x00, 0x14, 0xff]),
        ScriptType::P2WPKH
    ));
    assert!(matches!(
        classify_script(&[0xab, 0xcd]),
        ScriptType::Unknown
    ));
}

#[test]
fn test_outpoint_destructuring() {
    let op = Outpoint("abcd1234".to_string(), 1);
    let Outpoint(txid, vout) = op;
    assert_eq!(txid, "abcd1234");
    assert_eq!(vout, 1);
}

#[test]
fn test_script_slice() {
    let mut script = vec![0x00, 0x14];
    script.extend(vec![0u8; 20]);
    let data = read_pushdata(&script);
    assert_eq!(data.len(), 20);
}

#[test]
fn test_wallet_balance_trait() {
    let wallet = TestWallet { confirmed: 1500 };
    assert_eq!(wallet.balance(), 1500);
}

#[test]
fn test_apply_fee() {
    let mut balance = 10000;
    apply_fee(&mut balance, 250);
    assert_eq!(balance, 9750);
}

#[test]
fn test_move_txid() {
    let original = "deadbeef".to_string();
    let result = move_txid(original);
    assert_eq!(result, "txid: deadbeef");
}

#[test]
fn test_opcode_parsing() {
    assert_eq!(Opcode::from_byte(0xac), Ok(Opcode::OpChecksig));
    assert_eq!(Opcode::from_byte(0x76), Ok(Opcode::OpDup));
    assert_eq!(
        Opcode::from_byte(0x00),
        Err("Invalid opcode: 0x00".to_string())
    );
}

#[test]
fn test_utxo_ownership() {
    let utxo = UTXO {
        txid: vec![0xaa, 0xbb],
        vout: 0,
        value: 1000,
    };
    assert_eq!(consume_utxo(utxo.clone()), utxo);
}
