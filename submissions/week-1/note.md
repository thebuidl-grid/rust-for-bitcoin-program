### 1. Bitcoin Core CLI confirmation
Bitcoin core node is running in regtest mode.

```bash
bitcoin-cli getblockchaininfo
```
<img width="1082" height="346" alt="getblockchaininfo" src="https://github.com/user-attachments/assets/59e94f6f-1206-41c7-af05-0b69f02f6efe" />

chain: `regtest`
blocks: `101`
This screenshot was taken after mining 101 blocks

### 2. Generate Blocks
101 blocks generated for a target address
```zsh
bitcoin-cli -regtest generatetoaddress 101 "$(bitcoin-cli -regtest getnewaddress)"
```
Returns an error: Which tells the user that a default wallet is not automatically created

<img width="1086" height="122" alt="loadingwalletpt1" src="https://github.com/user-attachments/assets/27b8a364-4c95-48fe-a392-551b092f6627" />

Here you either create a wallet or load an existing wallet
Initially, I had created a wallet with
```zsh
bitcoin-cli createwallet btcwallet
```
So I loaded it into the program by using
```zsh
bitcoin-cli loadwallet btcwallet
```
Ouput = successful:

<img width="1086" height="70" alt="loadwalletpt2" src="https://github.com/user-attachments/assets/87a26d7d-28b0-4665-a971-32f7fe63ccbd" />
And now we are set to use command to generate 101 blocks
```zsh
bitcoin-cli -regtest generatetoaddress 101 "$(bitcoin-cli -regtest getnewaddress)"
```
Note that I had generated 101 blocks earlier during the learning session

<img width="1086" height="853" alt="generatetoaddress" src="https://github.com/user-attachments/assets/278d1ec5-0318-4890-98cb-01e38cf4a7ea" />

Verified new block height with `getblockcount`
```zsh
bitcoin-cli getblockcount
```
<img width="779" height="37" alt="getblockcount" src="https://github.com/user-attachments/assets/a37fba81-b84c-4caf-be11-5a50540e90f1" />

As you can see, it's +101 to the previous 101 blocks == 202.

### 3. Explore the Blockchain: Learn the structure and metadata of Bitcoin blocks.
Using the three commands specified:
```zsh
bitcoin-cli getbestblockhash
```
<img width="779" height="37" alt="getbestblockhash" src="https://github.com/user-attachments/assets/cee611e6-64b0-47b2-8e00-551a1f4110bc" />

```zsh
bitcoin-cli getblock 32ba733bbf4df4916da2aa2726c426ad59796522bc589133e2deaa81253d8df4
```
<img width="1036" height="412" alt="getblockpt1" src="https://github.com/user-attachments/assets/6e479c2b-c597-446c-adbe-4e125d478fd0" />

```zsh
bitcoin-cli getblockheader 32ba733bbf4df4916da2aa2726c426ad59796522bc589133e2deaa81253d8df4
```
<img width="1036" height="313" alt="getblockheaderpt1" src="https://github.com/user-attachments/assets/33c07590-ccfd-410d-a402-f4d9d2fd2b3d" />

All a success...

### 4. Work with Wallets: Get comfortable managing multiple wallets and addresses.
Created a new wallet using
```zsh
bitcoin-cli -regtest createwallet "testwallet"
```
<img width="1036" height="74" alt="Screenshot 2025-10-20 at 4 01 00 PM" src="https://github.com/user-attachments/assets/bd3c153e-2ce8-407e-bce6-cfe3374eb119" />

Generating few addresses
```zsh
for i in {1..5}; do bitcoin-cli -regtest getnewaddress; done
```
or 
```zsh
sh submissions/week-1/regen.sh
```
<img width="1036" height="103" alt="Screenshot 2025-10-20 at 4 44 07 PM" src="https://github.com/user-attachments/assets/c82faabb-504f-4bab-901e-26c85ec42bba" />

Listing all addresses
```zsh
bitcoin-cli listreceivedbyaddress 0 true
```
here we pass in two arguments on the minimum number of confirmations before payments are included, and if empty addresses (no payments) should be included
<img width="1036" height="835" alt="listaddresses" src="https://github.com/user-attachments/assets/e5f53e86-4cf9-400f-ab3d-417155b2ba70" />

Throws an error if two wallets are loaded
Using the first command to create a "testwallet", I had two wallets loaded.

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
