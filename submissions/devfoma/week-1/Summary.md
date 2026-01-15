## Reflection: Summary of Learning and Debugging Process

### What I Learned

Through this first week, I gained hands-on experience with how Bitcoin fundamentally operates beneath the surface. By using **regtest mode**, I was able to create a private, local blockchain environment to experiment safely with transactions and wallets.
I learned how to:

* Start and interact with a Bitcoin node using the command line.
* Generate blocks and understand **coinbase maturity**.
* Explore block structure, headers, and transaction metadata.
* Create, load, and manage multiple wallets in a regtest setup.
* Simulate sending BTC between wallets and confirm those transactions.
* Inspect **UTXOs (Unspent Transaction Outputs)** to understand how Bitcoin tracks spendable coins.
* Decode raw transactions to see the underlying data and structure of Bitcoin’s transaction model.

By the end of this process, I developed a solid foundational understanding of how nodes, wallets, blocks, and transactions all connect in the Bitcoin network.


### Challenges Faced

1. **Connection Issues with bitcoind**

   * Encountered repeated `Could not connect to the server 127.0.0.1:18443` errors.
   * Cause: `bitcoind` was not actively running or was using a different RPC port.
   * Solution: Started the daemon explicitly with

     ```bash
     bitcoind -regtest -daemon
     ```

     and verified connectivity with `bitcoin-cli -regtest getblockchaininfo`.

2. **Fee Estimation Error (Fallback Fee Disabled)**

   * Faced the error:

     ```
     Fee estimation failed. Fallbackfee is disabled.
     ```
   * Cause: Bitcoin Core couldn’t estimate a transaction fee in regtest mode.
   * Solution: Enabled fallback fees in the config file (`bitcoin.conf`) using

     ```
     fallbackfee=0.0001
     ```

     Then restarted the node.

3. **Wallet Not Loaded / Already Exists Errors**

   * Errors such as

     ```
     Requested wallet does not exist or is not loaded
     ```

     or

     ```
     Database already exists
     ```
   * Cause: Wallets were created but not properly loaded after restarting `bitcoind`.
   * Solution: Used

     ```bash
     bitcoin-cli -regtest loadwallet <walletname>
     ```

     to load them manually before running further commands.

4. **Transaction Not Found When Decoding**

   * Encountered

     ```
     No such mempool transaction. Use -txindex or provide a block hash
     ```
   * Cause: The transaction was already mined into a block, and `txindex` was not enabled.
   * Solution: Used the block hash from the mined block to retrieve and decode the transaction:

     ```bash
     bitcoin-cli -regtest getrawtransaction <txid> true <blockhash>
     ```

5. **Lock and Stop Errors**

   * Experienced `Cannot obtain a lock on directory` errors when trying to restart the node.
   * Solution: Used

     ```bash
     pkill -9 bitcoind
     ```

     to terminate the background process before restarting.


### Debugging Approach

Throughout these challenges, I focused on understanding the root cause of each issue instead of only copying commands.
My debugging approach included:

* Reading the exact error messages carefully.
* Checking running processes using system commands.
* Inspecting wallet directories to confirm their existence and paths.
* Consulting Bitcoin Core CLI help with `bitcoin-cli help <command>`.
* Sequentially testing each fix and verifying results before moving forward.

This process improved my confidence in navigating Bitcoin Core and taught me how to think methodically when dealing with blockchain infrastructure.


### Key Takeaway

Running a Bitcoin node locally was not just a technical exercise—it was a deep dive into how decentralized systems maintain integrity through clear structure and verifiable processes.
By setting up wallets, sending and decoding transactions, and debugging system-level issues, I now have a foundational, practical understanding of **Bitcoin’s UTXO model**, **block creation**, and **transaction lifecycle**, knowledge that will be critical before moving on to building tools in **Rust**.
