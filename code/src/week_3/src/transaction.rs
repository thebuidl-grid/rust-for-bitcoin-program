use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
pub struct Transaction {
    pub transaction_id: Txid,
    pub version: u32,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub lock_time: u32,
}

#[derive(Debug, Serialize)]
pub struct Input {
    pub txid: Txid, // [u8; 32],
    pub output_index: u32,
    pub script_sig: Vec<u8>,
    pub sequence: u32
}

#[derive(Debug, Serialize)]
pub struct Output {
    #[serde(serialize_with = "as_btc")]
    pub amount: Amount,
    pub script_pubkey: Vec<u8>,
}

fn as_btc<S: Serializer, T: BitcoinValue>(t: &T, s: S) -> Result<S::Ok, S::Error> {
    let btc = t.to_btc();
    s.serialize_f64(btc)
}

#[derive(Debug)]
pub struct Amount( u64);

impl Amount {
  // type associated functiion that initiate the instance of the struct i.e Amount
  pub fn from_sat(satoshi: u64) -> Amount {
    Amount(satoshi)
  }
}

trait BitcoinValue {
    fn to_btc(&self) -> f64;
}

impl BitcoinValue for Amount {
    fn to_btc(&self) -> f64 {
        self.0 as f64 / 100_000_000.0
    }
}


// https://learnmeabitcoin.com/technical/transaction/input/sequence/#:~:text=The%20sequence%20field%20can%20be,whilst%20it's%20in%20the%20mempool.


// 010000000001021fad2978f20da5173251fa87bb6ddecaa2255af7e7706517eb42e47b3de20bda0e00000000ffffffff4a60bd692c4cfe76e91569c5a2e29c823ea906c9eeac0d9817d3634293f3ccce0f00000000ffffffff017588e001000000001976a914133a81baeb8c8036b189e03c033ec9cbad96f94c88ac02483045022100d3341516a39dd88bb98c191f290e3e5098939af8461bd8e063ae70dd6b13597602207c72acc2487c08aeadeaab6bb43af92348588c37a50143b940839dee82a0c558012102030ef6b9a2eac63ef4d545534758ebae44e48a602939fff60cbef8ebe06eae4e02483045022100e4e96257d9d4abde83a4d9f287c5512151a7489faef12dcd5764ed3028e749e0022053c6704e9e6cd0a00209a95a9bbe3d2f9406412e862823f09f2cb8135897
// a72e012102030ef6b9a2eac63ef4d545534758ebae44e48a602939fff60cbef8ebe06eae4e00000000

#[derive(Debug)]
pub struct Txid([u8; 32]);

// [u8; 32] => array of 32 element each element is 1 byte [u8]; i.e one byte is u8;

impl Txid {
    pub fn from_bytes(bytes: [u8; 32]) -> Txid {
        Txid(bytes)
    }
}

impl Serialize for Txid {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut bytes = self.0.clone();
        bytes.reverse(); // reserve the byte in place
        s.serialize_str(&hex::encode(bytes))
    }
} 





 
// Bitcoin uses little-endian encoding for most of its numeric fields,
//  meaning the least significant byte comes first.

// 0100 = 9 

// 0010 = 1 


// 010000000001021fad2978f20da5173251fa87bb6ddecaa2255af7e7706517eb42e47b3de20bda0e00000000ffffffff4a60bd692c4cfe76e91569c5a2e29c823ea906c9eeac0d9817d3634293f3ccce0f00000000ffffffff017588e001000000001976a914133a81baeb8c8036b189e03c033ec9cbad96f94c88ac02483045022100d3341516a39dd88bb98c191f290e3e5098939af8461bd8e063ae70dd6b13597602207c72acc2487c08aeadeaab6bb43af92348588c37a50143b940839dee82a0c558012102030ef6b9a2eac63ef4d545534758ebae44e48a602939fff60cbef8ebe06eae4e02483045022100e4e96257d9d4abde83a4d9f287c5512151a7489faef12dcd5764ed3028e749e0022053c6704e9e6cd0a00209a95a9bbe3d2f9406412e862823f09f2cb8135897a72e012102030ef6b9a2eac63ef4d545534758ebae44e48a602939fff60cbef8ebe06eae4e00000000

// 02000000000101630b3e9da4770372a86e8b1934bf871770f92ee1dc1cc8d5a26844f4138e8ac60000000000ffffffff02a91a000000000000160014550cb3f40ce38b9b7cefda9ad219c88d20945f65e7180000000000001600140687444472fdbebae65e70c0483279ecfe30efd50247304402207ea731a870df2c64fa67d67f8741b6baeaaac98c4e065ed8787baa9551b5ee6e02202776a2bae842a0d3a00fb449f8ba439a34d832f84c50bc6ec0bcd9403954c01701210325642cfe29bb9e26203294b5395e7c5f491524636ee8ded36146ca2a6ae11ffd00000000