use crate::blockchain::Blockchain;
use crate::block::Block;
use crate::transaction::{Transaction, TxInput, TxOutput};
use crate::mempool::Mempool;

/// A local Bitcoin node simulator
#[derive(Debug)]
pub struct Node {
    /// The blockchain
    pub blockchain: Blockchain,
    /// Local mempool for pending transactions
    pub mempool: Mempool,
    /// Mining address (where block rewards go)
    pub mining_address: String,
}

impl Node {
    /// Create a new node
    pub fn new(mining_address: String) -> Self {
        Self {
            blockchain: Blockchain::new(),
            mempool: Mempool::new(),
            mining_address,
        }
    }

    /// Submit a transaction to the mempool
    pub fn submit_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        self.mempool.add_transaction(tx, &self.blockchain.utxo_set)
    }

    /// Create a new transaction
    pub fn create_transaction(
        &self,
        inputs: Vec<TxInput>,
        outputs: Vec<TxOutput>,
    ) -> Transaction {
        Transaction::new(inputs, outputs)
    }

    /// Mine a new block from pending transactions
    pub fn mine_block(&mut self) -> Result<Block, String> {
        // Get transactions from mempool (limit to reasonable number)
        let mut transactions = self.mempool.get_transactions_for_block(10);

        // Create coinbase transaction (mining reward)
        let coinbase = Transaction::new(
            vec![],
            vec![TxOutput {
                amount: 50_0000_0000, // 50 BTC reward (simplified - no halving)
                address: self.mining_address.clone(),
            }],
        );

        // Coinbase must be first
        transactions.insert(0, coinbase);

        // Create new block
        let prev_hash = self.blockchain.latest_hash();
        let mut block = Block::new(prev_hash, transactions, self.blockchain.difficulty);

        // Mine the block (find valid nonce)
        println!("Mining block...");
        block.mine();
        println!("Block mined! Hash: {}", block.hash());

        // Add block to blockchain
        self.blockchain.add_block(block.clone())?;

        // Remove included transactions from mempool
        let tx_ids: Vec<String> = block.transactions.iter().skip(1).map(|tx| tx.id()).collect();
        self.mempool.remove_transactions(&tx_ids);

        Ok(block)
    }

    /// Get blockchain height
    pub fn height(&self) -> usize {
        self.blockchain.height()
    }

    /// Get balance for an address
    pub fn get_balance(&self, address: &str) -> u64 {
        self.blockchain.get_balance(address)
    }

    /// Validate the entire blockchain
    pub fn validate_chain(&self) -> Result<(), String> {
        self.blockchain.is_valid()
    }

    /// Get pending transaction count
    pub fn pending_transactions(&self) -> usize {
        self.mempool.len()
    }

    /// Get blockchain info
    pub fn info(&self) -> NodeInfo {
        NodeInfo {
            height: self.height(),
            latest_hash: self.blockchain.latest_hash(),
            pending_transactions: self.pending_transactions(),
            difficulty: self.blockchain.difficulty,
        }
    }
}

/// Node information
#[derive(Debug)]
pub struct NodeInfo {
    pub height: usize,
    pub latest_hash: String,
    pub pending_transactions: usize,
    pub difficulty: u32,
}


