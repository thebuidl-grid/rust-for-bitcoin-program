use mini_bitcoin_node::{Node, Transaction, TxInput, TxOutput};

fn main() {
    println!("=== Mini Bitcoin Node Simulator ===\n");

    // Create a new node
    let mut node = Node::new("miner1".to_string());

    println!("1. Created new node with genesis block");
    println!("   Blockchain height: {}", node.height());
    println!("   Latest block hash: {}\n", node.blockchain.latest_hash());

    // Validate the chain
    match node.validate_chain() {
        Ok(()) => println!("✓ Blockchain is valid\n"),
        Err(e) => println!("✗ Blockchain validation failed: {}\n", e),
    }

    // Create some transactions
    println!("2. Creating transactions...");

    // Transaction 1: Send from genesis to address1
    let tx1 = Transaction::new(
        vec![TxInput {
            prev_tx_id: node.blockchain.blocks[0].transactions[0].id(),
            output_index: 0,
            signature: "sig1".to_string(),
        }],
        vec![
            TxOutput {
                amount: 30_0000_0000, // 30 BTC
                address: "address1".to_string(),
            },
            TxOutput {
                amount: 19_9999_9999, // ~20 BTC (change, minus 1 satoshi fee)
                address: "genesis".to_string(),
            },
        ],
    );

    println!("   Created transaction 1: {} -> address1 (30 BTC)", tx1.id());

    // Transaction 2: Send from address1 to address2
    let tx2 = Transaction::new(
        vec![TxInput {
            prev_tx_id: tx1.id(),
            output_index: 0,
            signature: "sig2".to_string(),
        }],
        vec![
            TxOutput {
                amount: 20_0000_0000, // 20 BTC
                address: "address2".to_string(),
            },
            TxOutput {
                amount: 9_9999_9999, // ~10 BTC (change)
                address: "address1".to_string(),
            },
        ],
    );

    println!("   Created transaction 2: address1 -> address2 (20 BTC)");

    // Submit transactions to mempool
    println!("\n3. Submitting transactions to mempool...");
    match node.submit_transaction(tx1.clone()) {
        Ok(()) => println!("   ✓ Transaction 1 added to mempool"),
        Err(e) => println!("   ✗ Failed to add transaction 1: {}", e),
    }

    match node.submit_transaction(tx2.clone()) {
        Ok(()) => println!("   ✓ Transaction 2 added to mempool"),
        Err(e) => println!("   ✗ Failed to add transaction 2: {}", e),
    }

    println!("   Pending transactions: {}\n", node.pending_transactions());

    // Mine a block
    println!("4. Mining block with pending transactions...");
    match node.mine_block() {
        Ok(block) => {
            println!("   ✓ Block mined successfully!");
            println!("   Block hash: {}", block.hash());
            println!("   Transactions in block: {}", block.transactions.len());
            println!("   Nonce: {}\n", block.nonce);
        }
        Err(e) => println!("   ✗ Failed to mine block: {}\n", e),
    }

    // Check balances
    println!("5. Checking balances...");
    println!("   Genesis balance: {} satoshis", node.get_balance("genesis"));
    println!("   Address1 balance: {} satoshis", node.get_balance("address1"));
    println!("   Address2 balance: {} satoshis", node.get_balance("address2"));
    println!("   Miner1 balance: {} satoshis\n", node.get_balance("miner1"));

    // Validate chain again
    println!("6. Validating blockchain after mining...");
    match node.validate_chain() {
        Ok(()) => println!("   ✓ Blockchain is still valid\n"),
        Err(e) => println!("   ✗ Blockchain validation failed: {}\n", e),
    }

    // Try to create an invalid transaction (double-spend)
    println!("7. Testing double-spend prevention...");
    let invalid_tx = Transaction::new(
        vec![TxInput {
            prev_tx_id: tx1.id(), // Already spent!
            output_index: 0,
            signature: "sig3".to_string(),
        }],
        vec![TxOutput {
            amount: 30_0000_0000,
            address: "attacker".to_string(),
        }],
    );

    match node.submit_transaction(invalid_tx) {
        Ok(()) => println!("   ✗ Double-spend was allowed (BUG!)"),
        Err(e) => println!("   ✓ Double-spend prevented: {}\n", e),
    }

    // Try to create a transaction with insufficient funds
    println!("8. Testing insufficient funds prevention...");
    let poor_tx = Transaction::new(
        vec![TxInput {
            prev_tx_id: tx2.id(),
            output_index: 0, // This output has 20 BTC
            signature: "sig4".to_string(),
        }],
        vec![TxOutput {
            amount: 30_0000_0000, // Trying to send 30 BTC with only 20 BTC
            address: "recipient".to_string(),
        }],
    );

    match node.submit_transaction(poor_tx) {
        Ok(()) => println!("   ✗ Insufficient funds transaction was allowed (BUG!)"),
        Err(e) => println!("   ✓ Insufficient funds prevented: {}\n", e),
    }

    // Node info
    println!("9. Node Information:");
    let info = node.info();
    println!("   Height: {}", info.height);
    println!("   Latest hash: {}", info.latest_hash);
    println!("   Pending transactions: {}", info.pending_transactions);
    println!("   Difficulty: {}\n", info.difficulty);

    println!("=== Demo Complete ===");
}


