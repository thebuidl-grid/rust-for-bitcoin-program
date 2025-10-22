### 1. Verify Your Node Setup: Ensure your node is correctly configured and responsive via CLI.
Confirm your Bitcoin Core node is running in regtest mode.
- Run `bitcoin-cli getblockchaininfo` and note the chain and blocks values.

![alt text](image.png)


### 2. Generate Blocks: Understand block creation and coinbase maturity on regtest.
- Use the command below to generate 101 blocks to a new address:
`bitcoin-cli -regtest generatetoaddress 101 "$(bitcoin-cli -regtest getnewaddress)"`
- Verify the new block height with getblockcount.
![alt text](image-1.png)
![alt text](image-2.png)

### 3. Explore the Blockchain: Learn the structure and metadata of Bitcoin blocks.
Use these commands to inspect your chain:
- `getbestblockhash`
- `getblock <blockhash>`
- `getblockheader <blockhash>`
![alt text](image-3.png)

### 4. Work with Wallets: Get comfortable managing multiple wallets and addresses.
- Create a new wallet: `bitcoin-cli -regtest createwallet "testwallet"`
- Generate a few new addresses and list them.
![alt text](image-4.png)


### 5. Send and Track Transactions: Learn how transactions are created, broadcast, and confirmed.
- Send test coins to a new address using sendtoaddress.
- Retrieve the transaction using gettransaction.
![alt text](image-5.png)
![alt text](image-6.png)

### 6. Inspect UTXOs: Send and Track Transactions: Understand how UTXOs represent spendable Bitcoin
- Use `listunspent` to view your spendable outputs.
- Record details like txid, vout, and amount.
![alt text](image-10.png)



### 7. Decode a Raw Transaction: Deepen understanding of Bitcoin’s transaction model.
- Fetch a transaction with `getrawtransaction <txid>` true.
- Explore the input/output details and note the structure.
![alt text](image-7.png)
![alt text](image-8.png)
![alt text](image-9.png)


### 8. Simulate a Payment Workflow: Experience the full transaction lifecycle in regtest.
- Create two wallets: `sender` and `receiver`.
- Send BTC from one to the other and confirm the transaction by mining a block.

![alt text](image-11.png)
![alt text](image-12.png)





### Summary Notes 
- Anytime restart the environment I need to relaod the wallets in the environment using the this piece of code 
for w in $(ls ~/.bitcoin/regtest/wallets); do
  bitcoin-cli -regtest loadwallet "$w"
done

- Also had to add  fallbackfee=0.0001 to the config file to cater to the transction fee for sending coin 

- to unload all wallets at once 
use: bitcoin-cli -regtest listwallets | tr -d '[],"' | xargs -n1 -I{} bitcoin-cli -regtest unloadwallet "{}"
