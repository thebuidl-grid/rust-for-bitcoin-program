# Week 1: Bitcoin Node Setup and Regtest Exploration

## Overview

This week focused on setting up and interacting with a **Bitcoin Core node in regtest mode**.
The goal was to understand how local blockchain environments work, how wallets and transactions operate, and how to debug errors while working with `bitcoind` and `bitcoin-cli`.

---

## 🧠 What I Did

1. **Started the node in regtest mode**

   ```bash
   bitcoind -regtest -daemon
   bitcoin-cli -regtest getblockchaininfo
   ```

   Verified the node was running locally with an empty chain.

2. **Generated 101 blocks**

   ```bash
   ADDR=$(bitcoin-cli -regtest getnewaddress)
   bitcoin-cli -regtest generatetoaddress 101 "$ADDR"
   ```

   This gave me 50 BTC * 101 blocks = 5050 BTC (not spendable until 100 confirmations).

3. **Explored blockchain details**

   * Checked current height (`getblockcount`)
   * Fetched best block hash and headers
   * Confirmed all block data stored properly

4. **Created and managed wallets**

   ```bash
   bitcoin-cli -regtest createwallet "testwallet"
   bitcoin-cli -regtest -rpcwallet=testwallet getnewaddress
   ```

   Learned how to create isolated wallets for testing.

5. **Attempted to send and track a transaction**

   ```bash
   RECV_ADDR=$(bitcoin-cli -regtest -rpcwallet=testwallet getnewaddress)
   TXID=$(bitcoin-cli -regtest -rpcwallet=testwallet sendtoaddress "$RECV_ADDR" 10.0)
   bitcoin-cli -regtest -rpcwallet=testwallet gettransaction "$TXID"
   ```

   ❌ Encountered error:

   ```
   error code: -18
   error message: Requested wallet does not exist or is not loaded
   ```

   The issue happened because only one wallet (`yooo`) was loaded at that time. `testwallet` had not been loaded into memory.

6. **Created sender and receiver wallets**

   ```bash
   bitcoin-cli -regtest createwallet "sender"
   bitcoin-cli -regtest createwallet "receiver"
   ```

   Then generated new addresses and mined blocks to fund the sender.

7. **Second transaction test**

   * Sent funds from sender to receiver
   * Mined one block to confirm the transaction
   * Used `gettransaction` to verify transaction details

---

## 🧩 Mistakes and Fixes

* **Wallet not loaded error (-18):** Forgot to load `testwallet` using `loadwallet testwallet`.
* **Transaction lookup error (-5):** Occurred because `-txindex` flag was not enabled, so historical transaction data couldn’t be retrieved.
* Learned how to fix both by loading wallets explicitly and starting `bitcoind` with `-txindex=1`.

---

## 💡 Key Takeaways

* Regtest allows isolated blockchain experimentation with full control.
* Every wallet must be explicitly created or loaded before use.
* Transaction indexes (`-txindex`) are crucial for querying historical data.
* Understanding these interactions lays the foundation for scripting Bitcoin workflows and backend integrations.
