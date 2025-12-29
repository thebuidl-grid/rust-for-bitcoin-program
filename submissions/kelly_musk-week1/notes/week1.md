# Week 1 Focus — Bitcoin Fundamentals
Before diving into Rust, we’ll start with understanding Bitcoin from first principles. By the end of this week, you should be able to:
- Run a Bitcoin node in regtest mode
- Explore blocks, transactions, and the mempool
- Simulate simple transactions between wallets
- Understand how UTXOs power Bitcoin


### 1. Verify Your Node Setup: Ensure your node is correctly configured and responsive via CLI.
Confirm your Bitcoin Core node is running in regtest mode.
- Run `bitcoin-cli getblockchaininfo` and note the chain and blocks values.

### my solution: 

![Screenshot for Node Setup Verification](/submissions/kelly_musk-week1/public/blockchaininfo.png "Verify Node Setup")

### 2. Generate Blocks: Understand block creation and coinbase maturity on regtest.
- Use the command below to generate 101 blocks to a new address:
`bitcoin-cli -regtest generatetoaddress 101 "$(bitcoin-cli -regtest getnewaddress)"`
- Verify the new block height with getblockcount.

### my solution:

![Screenshot for Block Creation](/submissions/kelly_musk-week1/public/generated%20first%20blocks.png "Block Creation")

### 3. Explore the Blockchain: Learn the structure and metadata of Bitcoin blocks.
Use these commands to inspect your chain:
- `getbestblockhash`
- `getblock <blockhash>`
- `getblockheader <blockhash>`

### my solution:

- `bitcoin-cli -regtest getbestblockhash`

! [] (/submissions/kelly_musk-week1/public/bestblockhash.png) "Verify Node Setup"

### 4. Work with Wallets: Get comfortable managing multiple wallets and addresses.
- Create a new wallet: `bitcoin-cli -regtest createwallet "testwallet"`
- Generate a few new addresses and list them.

! [](/submissions/kelly_musk-week1/public/create%20first%20wallet.png) "Creating and Listing new wallets"

### 5. Send and Track Transactions: Learn how transactions are created, broadcast, and confirmed.
- Send test coins to a new address using sendtoaddress.
- Retrieve the transaction using gettransaction.


![](/submissions/kelly_musk-week1/public/created%20second%20wallet.png)

 ![](/submissions/kelly_musk-week1/public/generated%20a%20block%20after%20sending%20to%20an%20address.png)

### 6. Inspect UTXOs: Send and Track Transactions: Understand how UTXOs represent spendable Bitcoin
- Use `listunspent` to view your spendable outputs.
- Record details like txid, vout, and amount.

### my solution:

 ![](/submissions/kelly_musk-week1/public/listof%20unspent%20utxo.png)

### 7. Decode a Raw Transaction: Deepen understanding of Bitcoin’s transaction model.
- Fetch a transaction with `getrawtransaction <txid>` true.
- Explore the input/output details and note the structure.



![](/submissions/kelly_musk-week1/public/got%20my%20first%20raw%20transaction%20hex%20value.png "Decoding a Raw TXN")
![](/submissions/kelly_musk-week1/public/decoded%20atransaxtion.png)

### 8. Simulate a Payment Workflow: Experience the full transaction lifecycle in regtest.
- Create two wallets: `sender` and `receiver`.
- Send BTC from one to the other and confirm the transaction by mining a block.

![](/submissions/kelly_musk-week1/public/how%20i%20sent%20btc%20to%20%20my%20testwallet.png)

![Screenshot for Mining and Transaction Confirmation](/submissions/kelly_musk-week1/public/my%20balance%20after%20sendidg%20to%20a%20wallet%20.png "Mining and Transaction Confirmation")