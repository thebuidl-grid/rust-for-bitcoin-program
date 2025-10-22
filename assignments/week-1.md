# Week 1 Focus — Bitcoin Fundamentals
Before diving into Rust, we’ll start with understanding Bitcoin from first principles. By the end of this week, you should be able to:
- Run a Bitcoin node in regtest mode
- Explore blocks, transactions, and the mempool
- Simulate simple transactions between wallets
- Understand how UTXOs power Bitcoin


### 1. Verify Your Node Setup: Ensure your node is correctly configured and responsive via CLI.
Confirm your Bitcoin Core node is running in regtest mode.
- Run `bitcoin-cli getblockchaininfo` and note the chain and blocks values.


### 2. Generate Blocks: Understand block creation and coinbase maturity on regtest.
- Use the command below to generate 101 blocks to a new address:
<!-- `bitcoin-cli -regtest generatetoaddress 101 "$(bitcoin-cli -regtest getnewaddress)"` -->
- Verify the new block height with getblockcount.


### 3. Explore the Blockchain: Learn the structure and metadata of Bitcoin blocks.
Use these commands to inspect your chain:
- `getbestblockhash`
- `getblock <blockhash>`
- `getblockheader <blockhash>`


### 4. Work with Wallets: Get comfortable managing multiple wallets and addresses.
- Create a new wallet: `bitcoin-cli -regtest createwallet "testwallet"`
- Generate a few new addresses and list them.

### 5. Send and Track Transactions: Learn how transactions are created, broadcast, and confirmed.
- Send test coins to a new address using sendtoaddress.
- Retrieve the transaction using gettransaction.



### 6. Inspect UTXOs: Send and Track Transactions: Understand how UTXOs represent spendable Bitcoin
- Use `listunspent` to view your spendable outputs.
- Record details like txid, vout, and amount.

### 7. Decode a Raw Transaction: Deepen understanding of Bitcoin’s transaction model.
- Fetch a transaction with `getrawtransaction <txid>` true.
- Explore the input/output details and note the structure.

### 8. Simulate a Payment Workflow: Experience the full transaction lifecycle in regtest.
- Create two wallets: `sender` and `receiver`.
- Send BTC from one to the other and confirm the transaction by mining a block.

---

# How to Submit Your Assignment
Follow these steps carefully to submit your work:
- Fork this repository to your own GitHub account.
- Create a new branch for your submission:
`git checkout -b week-1-assignment`
- Add your work inside the appropriate `submissions` folder:
```
submissions/
    └── week-1/
        ├── note.md/

```
- Commit and push your changes:
```
git add .
git commit -m "chore: submit assignment 1"
git push origin week-1-assignment
```

- Create a Pull Request (PR) to the main repository. Your PR should include:
    - A brief summary of what you learned
    - Screenshots or logs of your node setup and transaction results


# Evaluation Criteria
Submissions will be evaluated based on:
- Completeness and correctness of your assignment
- Clarity of documentation (notes and code comments)
- Consistency in following submission structure
