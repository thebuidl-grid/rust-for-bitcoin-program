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
Using the first command to create a "testwallet", I had two wallets loaded. (about earlier I talked about loading a wallet); So I had to unload using
```zsh
bitcoin-cli unloadwallet testwallet
```
<img width="867" height="56" alt="Screenshot 2025-10-20 at 5 14 49 PM" src="https://github.com/user-attachments/assets/95142d6f-042c-4ab6-b5f3-efb5953f0ea2" />

Then everything else went smoothly.

### 5. Sending and Tracking Transactions
Unloaded my "btcwallet" and loaded my "testwallet" using
```zsh
bitcoin-cli unloadwallet btcwallet && bitcoin-cli loadwallet testwallet
```
I generates an address for testwallet and checked the balance of the testwallet using `getbalance`. Returned `0`
Now we must send some amount of btc from my "btcwallet" to this "testwallet", so we unload and load again, back to "btcwallet" (btcwallet has generated 202 blocks, so it has huge amount of funds)

<img width="867" height="324" alt="Screenshot 2025-10-20 at 8 32 24 PM" src="https://github.com/user-attachments/assets/847974d0-c78b-4441-90c1-a3808e52b6a8" />

Sending funds, we use
```zsh
bitcoin-cli sendtoaddress bcrt1qudk2dcd27mqt9whlhw6mqnmlkxmztt0r3j8ukz 50
```
Got an error

<img width="867" height="74" alt="fallbackfee err" src="https://github.com/user-attachments/assets/af5f4eb5-2e93-40de-b0fa-b8d793ab955e" />

here, in the `bitcoin.conf` file, I set the `fallbackfee` to any arbitrary value, stopped the daemon, restarted it, and loaded my "btcwallet"

Now, it passes

<img width="867" height="191" alt="sendtoaddress" src="https://github.com/user-attachments/assets/033c303f-39c2-42a1-8d7a-0ab146b2aa7b" />

To inspect the transaction
```zsh
bitcoin-cli gettransaction a6cd785bbc5e4cddbf56d1342fe01ece18638cee23ae61d2d05fc9c0be341fdc
```
<img width="1440" height="581" alt="gettransaction" src="https://github.com/user-attachments/assets/13bfcd66-faf3-41e2-95a0-1691dfd27306" />

using `getbalance` for "testwallet" (the receiver), it returns `0`
Using
```zsh
bitcoin-cli listreceivedbyaddress 0 true
```
You can inspect the updated balance. That said, if you're not still convinced, then you must generate one block for the transaction to be confirmed for the receiver address... hereby `getbalance` then returns the appropriate balance

### 6. Inspect UTXOs: Send and Track Transactions
```zsh
bitcoin-cli listunspent
```
<img width="1440" height="326" alt="Screenshot 2025-10-20 at 9 29 53 PM" src="https://github.com/user-attachments/assets/389be18f-97f9-4c87-909a-78d0197da5b2" />

### 7. Decode a Raw Transaction: Deepen understanding of Bitcoin’s transaction model.
By setting `txindex=1` in the bitcoin.conf file, the error code of -5 is cleared into a successful result
```zsh
bitcoin-cli getrawtransaction a6cd785bbc5e4cddbf56d1342fe01ece18638cee23ae61d2d05fc9c0be341fdc true
```
<img width="1440" height="868" alt="Screenshot 2025-10-20 at 9 55 52 PM" src="https://github.com/user-attachments/assets/d3160cdb-ce98-49a5-8eb2-ad25abdfd143" />

<img width="1440" height="345" alt="Screenshot 2025-10-20 at 9 56 28 PM" src="https://github.com/user-attachments/assets/037e047b-ccc0-4702-8644-1878c7ca175b" />


### 8. Simulate a Payment Workflow: 

This task has already been done in Task #5 exactly between "btcwallet" and "testwallet"

Gracias.
