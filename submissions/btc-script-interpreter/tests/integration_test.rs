use btc_script_interpreter::{
    interpreter::Interpreter,
    opcodes::Opcode,
    stack::Stack,
    transaction::{create_p2pkh_script, is_p2pkh_script, Transaction, TxInput, TxOutput},
};

#[test]
fn test_stack_operations() {
    let mut stack = Stack::new();
    stack.push(vec![1, 2, 3]);
    assert_eq!(stack.len(), 1);

    stack.dup().unwrap();
    assert_eq!(stack.len(), 2);

    let top = stack.pop().unwrap();
    assert_eq!(top, vec![1, 2, 3]);
}

#[test]
fn test_op_dup() {
    let mut stack = Stack::new();
    stack.push(vec![42]);

    let opcode = Opcode::OpDup;
    let mut pos = 0;
    opcode.execute(&mut stack, &[], &mut pos, &[]).unwrap();

    assert_eq!(stack.len(), 2);
}

#[test]
fn test_op_hash160() {
    let mut stack = Stack::new();
    let data = b"hello world";
    stack.push(data.to_vec());

    let opcode = Opcode::OpHash160;
    let mut pos = 0;
    opcode.execute(&mut stack, &[], &mut pos, &[]).unwrap();

    let result = stack.pop().unwrap();
    assert_eq!(result.len(), 20);
}

#[test]
fn test_op_equalverify_success() {
    let mut stack = Stack::new();
    stack.push(vec![1, 2, 3]);
    stack.push(vec![1, 2, 3]);

    let opcode = Opcode::OpEqualVerify;
    let mut pos = 0;
    let result = opcode.execute(&mut stack, &[], &mut pos, &[]);

    assert!(result.is_ok());
    assert_eq!(stack.len(), 0);
}

#[test]
fn test_op_equalverify_failure() {
    let mut stack = Stack::new();
    stack.push(vec![1, 2, 3]);
    stack.push(vec![4, 5, 6]);

    let opcode = Opcode::OpEqualVerify;
    let mut pos = 0;
    let result = opcode.execute(&mut stack, &[], &mut pos, &[]);

    assert!(result.is_err());
}

#[test]
fn test_simple_script_execution() {
    let mut interpreter = Interpreter::new(false);

    let script = vec![0x01, 0x42];
    let sighash = vec![0u8; 32];

    let result = interpreter.execute(&script, &sighash).unwrap();
    assert!(result);
}

#[test]
fn test_p2pkh_script_creation() {
    let pubkey_hash = vec![1u8; 20];
    let script = create_p2pkh_script(&pubkey_hash).unwrap();

    assert_eq!(script.len(), 25);
    assert!(is_p2pkh_script(&script));

    assert_eq!(script[0], 0x76);
    assert_eq!(script[1], 0xa9);
    assert_eq!(script[2], 0x14);
    assert_eq!(script[23], 0x88);
    assert_eq!(script[24], 0xac);
}

#[test]
fn test_p2pkh_script_validation() {
    let valid = vec![
        0x76, 0xa9, 0x14, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        0x88, 0xac,
    ];
    assert!(is_p2pkh_script(&valid));

    let invalid_length = vec![0x76, 0xa9, 0x14];
    assert!(!is_p2pkh_script(&invalid_length));

    let invalid_opcodes = vec![
        0x76, 0xa9, 0x14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x87, 0xac,
    ];
    assert!(!is_p2pkh_script(&invalid_opcodes));
}

#[test]
fn test_transaction_deserialization() {
    let pubkey_hash = vec![1u8; 20];
    let script_pubkey = create_p2pkh_script(&pubkey_hash).unwrap();

    let tx = Transaction {
        version: 1,
        inputs: vec![TxInput {
            prev_tx: [0u8; 32],
            prev_index: 0,
            script_sig: vec![0x01, 0x42],
            sequence: 0xffffffff,
        }],
        outputs: vec![TxOutput {
            value: 5000000000,
            script_pubkey: script_pubkey.clone(),
        }],
        locktime: 0,
    };

    let serialized = tx.serialize().unwrap();
    assert!(!serialized.is_empty());

    let deserialized = Transaction::deserialize(&serialized).unwrap();
    assert_eq!(deserialized.version, tx.version);
    assert_eq!(deserialized.inputs.len(), tx.inputs.len());
    assert_eq!(deserialized.outputs.len(), tx.outputs.len());
}

#[test]
fn test_invalid_transaction_empty_scriptsig() {
    let pubkey_hash = vec![1u8; 20];
    let script_pubkey = create_p2pkh_script(&pubkey_hash).unwrap();

    let tx = Transaction {
        version: 1,
        inputs: vec![TxInput {
            prev_tx: [0u8; 32],
            prev_index: 0,
            script_sig: vec![],
            sequence: 0xffffffff,
        }],
        outputs: vec![TxOutput {
            value: 5000000000,
            script_pubkey: vec![],
        }],
        locktime: 0,
    };

    let result = tx.validate_p2pkh(0, &script_pubkey, false);
    assert!(result.is_err() || result.unwrap() == false);
}

#[test]
fn test_sighash_computation() {
    let pubkey_hash = vec![1u8; 20];
    let script_pubkey = create_p2pkh_script(&pubkey_hash).unwrap();

    let tx = Transaction {
        version: 1,
        inputs: vec![TxInput {
            prev_tx: [0u8; 32],
            prev_index: 0,
            script_sig: vec![],
            sequence: 0xffffffff,
        }],
        outputs: vec![TxOutput {
            value: 5000000000,
            script_pubkey: script_pubkey.clone(),
        }],
        locktime: 0,
    };

    let sighash = tx.compute_sighash(0, &script_pubkey).unwrap();
    assert_eq!(sighash.len(), 32);
}
