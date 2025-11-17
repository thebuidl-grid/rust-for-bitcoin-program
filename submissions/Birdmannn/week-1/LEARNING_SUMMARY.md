# Week 1 Learning Summary

## Overview

This week provided comprehensive hands-on experience with Bitcoin Core in regtest mode, covering blockchain exploration, wallet management, transaction creation, and the complete payment workflow.

## Key Learnings

### 1. Bitcoin Core Setup & CLI Basics

- Successfully set up and confirmed Bitcoin Core running in **regtest mode** (regression test network)
- Learned to use `bitcoin-cli` commands to interact with the node
- Verified blockchain state using `getblockchaininfo` and `getblockcount`

### 2. Block Generation & Mining

- Understood that Bitcoin Core requires a wallet to be loaded before performing operations
- Learned the difference between creating and loading wallets
- Successfully generated 101 blocks to a target address using `generatetoaddress`
- Verified block generation by checking block height increments (from 101 to 202 blocks)

### 3. Blockchain Structure & Exploration

- Explored block metadata using three key commands:
  - `getbestblockhash` - retrieves the hash of the most recent block
  - `getblock` - provides detailed block information including transactions and metadata
  - `getblockheader` - shows block header details without full transaction data
- Gained understanding of how blocks are structured and linked in the blockchain

### 4. Wallet Management

- Created and managed multiple wallets in regtest (`btcwallet` and `testwallet`)
- Generated new addresses using `getnewaddress`
- Listed all addresses with `listreceivedbyaddress` to view wallet contents
- Learned that only one wallet can be active at a time
- Discovered `unloadwallet` command to switch between wallets
- Practiced wallet operations through both CLI commands and shell scripts

### 5. Sending and Tracking Transactions

- Successfully sent 50 BTC from `btcwallet` to `testwallet` using `sendtoaddress`
- Encountered and resolved the `fallbackfee` error by configuring `bitcoin.conf`
- Used `gettransaction` to inspect transaction details (txid, inputs, outputs, fees)
- Learned that unconfirmed transactions show in `listreceivedbyaddress` but not in `getbalance`
- Understood that transactions need block confirmation for balance updates
- Verified transaction confirmation by mining a block

### 6. Inspect UTXOs (Unspent Transaction Outputs)

- Used `listunspent` to view all spendable outputs in the wallet
- Learned to identify key UTXO properties: txid, vout (output index), and amount
- Understood how UTXOs represent the spendable Bitcoin in a wallet

### 7. Decode Raw Transactions

- Configured `txindex=1` in `bitcoin.conf` to enable transaction indexing
- Used `getrawtransaction <txid> true` to retrieve and decode raw transaction data
- Explored detailed transaction structure including inputs, outputs, and metadata
- Learned how to inspect transaction details at a lower level

### 8. Payment Workflow Simulation

- Successfully completed a full payment workflow between two wallets:
  - Created sender (`btcwallet`) and receiver (`testwallet`) wallets
  - Sent BTC from sender to receiver
  - Confirmed transaction by mining a block
  - Verified balance updates in receiver wallet

## Tools & Commands Mastered

- `bitcoin-cli getblockchaininfo` - blockchain state
- `bitcoin-cli generatetoaddress` - mine blocks to address
- `bitcoin-cli getblockcount` - current block height
- `bitcoin-cli getbestblockhash` - latest block hash
- `bitcoin-cli getblock` - block details
- `bitcoin-cli getblockheader` - block header info
- `bitcoin-cli createwallet` - create new wallet
- `bitcoin-cli loadwallet` - load existing wallet
- `bitcoin-cli unloadwallet` - unload wallet
- `bitcoin-cli getnewaddress` - generate addresses
- `bitcoin-cli listreceivedbyaddress` - list wallet addresses
- `bitcoin-cli getbalance` - check wallet balance
- `bitcoin-cli sendtoaddress` - send BTC to address
- `bitcoin-cli gettransaction` - inspect transaction details
- `bitcoin-cli listunspent` - view unspent outputs
- `bitcoin-cli getrawtransaction` - decode raw transaction data

## Configuration Insights

- **fallbackfee**: Required in `bitcoin.conf` for transaction broadcasting in regtest
- **txindex**: Set to `1` in `bitcoin.conf` to enable full transaction indexing for `getrawtransaction`

## Key Concepts Understood

- **Regtest Mode**: Isolated testing environment for Bitcoin development
- **Wallet Management**: Multiple wallets can exist but only one can be active
- **Transaction Lifecycle**: Creation → Broadcasting → Confirmation (via mining)
- **UTXO Model**: Bitcoin's spending model based on unspent transaction outputs
- **Block Confirmation**: Transactions need to be included in a mined block to be confirmed
- **Transaction Structure**: Raw transactions contain inputs, outputs, and metadata
