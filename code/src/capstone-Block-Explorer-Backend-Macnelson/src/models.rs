use serde::Serialize;

// JSON responses for the API
#[derive(Serialize)]
pub struct BlockResponse {
    pub hash: String,
    pub height: u32,
    pub version: u32,
    pub prev_block: String,
    pub merkle_root: String,
    pub timestamp: u32,
    pub bits: u32,
    pub nonce: u32,
    pub tx_count: usize,
    pub size: usize,
}

#[derive(Serialize)]
pub struct TxResponse {
    pub txid: String,
    pub version: u32,
    pub lock_time: u32,
    pub block_hash: Option<String>,
    pub block_height: Option<u32>,
    pub confirmations: Option<u64>,
    pub inputs: Vec<TxInSimplified>,
    pub outputs: Vec<TxOutSimplified>,
    pub size: usize,
    pub vsize: usize,
    pub weight: usize,
}

#[derive(Serialize, serde::Deserialize, Default)]
pub struct TxInSimplified {
    pub prev_txid: String,
    pub vout: u32,
    pub script_sig: String,
    pub sequence: u32,
    pub witness: Vec<String>,
}

#[derive(Serialize, serde::Deserialize, Default)]
pub struct TxOutSimplified {
    pub value: u64,
    pub script_pubkey: String,
}
#[derive(Serialize)]
pub struct LatestBlocksResponse {
    pub blocks: Vec<BlockSummary>,
    pub total_count: u32,
}

#[derive(Serialize)]
pub struct BlockSummary {
    pub hash: String,
    pub height: u32,
    pub timestamp: u32,
    pub tx_count: usize,
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub total_blocks: u32,
    pub total_transactions: u64,
    pub latest_block_height: u32,
    pub latest_block_hash: String,
}