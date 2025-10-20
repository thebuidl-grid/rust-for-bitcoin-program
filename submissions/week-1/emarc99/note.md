# Week 1 Assignment: Bitcoin Fundamentals - Technical Report

- **Author:** emarc99
- **Environment:** Bitcoin Core on WSL (WSL2)
- **Date:** October 20, 2025
- **Assignment:** Bitcoin Node Operations and Transaction Analysis

---

## Executive Summary

This report documents the comprehensive exploration of Bitcoin Core's fundamental operations in regtest mode, covering node setup verification, blockchain exploration, wallet management, transaction creation and tracking, UTXO analysis, and complete payment workflow simulation. All operations were executed successfully, demonstrating proficiency in Bitcoin node operations and understanding of the underlying Bitcoin protocol mechanics.

---

## Table of Contents

1. [Node Setup and Verification](#1-node-setup-and-verification)
2. [Block Generation and Coinbase Maturity](#2-block-generation-and-coinbase-maturity)
3. [Blockchain Structure Exploration](#3-blockchain-structure-exploration)
4. [Wallet Management and Address Generation](#4-wallet-management-and-address-generation)
5. [Transaction Creation and Tracking](#5-transaction-creation-and-tracking)
6. [UTXO Analysis and Structure](#6-utxo-analysis-and-structure)
7. [Raw Transaction Decoding](#7-raw-transaction-decoding)
8. [Payment Workflow Simulation](#8-payment-workflow-simulation)
9. [Key Learnings and Observations](#9-key-learnings-and-observations)
10. [Technical Insights](#10-technical-insights)

---

## 1. Node Setup and Verification

### Objective
Confirm Bitcoin Core node is properly configured and responsive in regtest mode.

### Command Executed
```bash
bitcoin-cli -regtest getblockchaininfo
```

### Results
```json
{
  "chain": "regtest",
  "blocks": 101,
  "headers": 101,
  "bestblockhash": "3c2f3cdf8cd7b7754499eb52fdefaf4203b63788c50d4cc9bc0d310526738d35",
  "bits": "207fffff",
  "target": "7fffff0000000000000000000000000000000000000000000000000000000000",
  "difficulty": 4.656542373906925e-10,
  "time": 1760656845,
  "mediantime": 1760656844,
  "verificationprogress": 0.2404991063807714,
  "initialblockdownload": false,
  "chainwork": "00000000000000000000000000000000000000000000000000000000000000cc",
  "size_on_disk": 30375,
  "pruned": false,
  "warnings": []
}
```

### Analysis
- **Chain Type:** Successfully running on `regtest` network
- **Block Height:** 101 blocks (sufficient for coinbase maturity testing)
- **Difficulty:** Ultra-low difficulty (4.65e-10) - normal for regtest environment
- **Synchronization:** Node is fully synchronized (initialblockdownload: false)
- **Storage:** Efficient disk usage at 30,375 bytes
- **Network State:** No warnings, indicating stable operation

### Verification Command
```bash
bitcoin-cli -regtest getblockcount
# Output: 101
```

---

## 2. Block Generation and Coinbase Maturity

### Objective
Understand block creation process and coinbase maturity rules in Bitcoin.

### Context
Bitcoin requires newly mined coins (coinbase rewards) to mature for 100 blocks before they can be spent. In regtest, I generated 101 blocks to ensure at least one coinbase transaction is spendable.

### Command Pattern
```bash
bitcoin-cli -regtest generatetoaddress 101 "$(bitcoin-cli -regtest getnewaddress)"
```

### Best Block Hash
```bash
bitcoin-cli -regtest getbestblockhash
# Output: 3c2f3cdf8cd7b7754499eb52fdefaf4203b63788c50d4cc9bc0d310526738d35
```

### Key Concepts Demonstrated
1. **Coinbase Maturity:** The first block's reward becomes spendable at block 101
2. **Block Generation:** Instant mining in regtest vs. 10-minute average on mainnet
3. **Blockchain Progression:** Linear chain construction with cryptographic linking

---

## 3. Blockchain Structure Exploration

### Objective
Deep dive into Bitcoin block structure, understanding headers, merkle roots, and block metadata.

### 3.1 Block Header Analysis

**Command:**
```bash
bitcoin-cli -regtest getblockheader 3c2f3cdf8cd7b7754499eb52fdefaf4203b63788c50d4cc9bc0d310526738d35
```

**Output:**
```json
{
  "hash": "3c2f3cdf8cd7b7754499eb52fdefaf4203b63788c50d4cc9bc0d310526738d35",
  "confirmations": 1,
  "height": 101,
  "version": 536870912,
  "versionHex": "20000000",
  "merkleroot": "6cf4ebb6136507dc722dc8afb584bc1370411b89f9fb5119082f4a455fc08843",
  "time": 1760656845,
  "mediantime": 1760656844,
  "nonce": 3,
  "bits": "207fffff",
  "difficulty": 4.656542373906925e-10,
  "chainwork": "00000000000000000000000000000000000000000000000000000000000000cc",
  "nTx": 1,
  "previousblockhash": "41bf4350eda2199c9a74d83e0b60dc09ef87eb736e57c443f5c7e93a79d411d7"
}
```

### 3.2 Full Block Details

**Command:**
```bash
bitcoin-cli -regtest getblock 3c2f3cdf8cd7b7754499eb52fdefaf4203b63788c50d4cc9bc0d310526738d35
```

**Output:**
```json
{
  "hash": "3c2f3cdf8cd7b7754499eb52fdefaf4203b63788c50d4cc9bc0d310526738d35",
  "confirmations": 1,
  "height": 101,
  "version": 536870912,
  "versionHex": "20000000",
  "merkleroot": "6cf4ebb6136507dc722dc8afb584bc1370411b89f9fb5119082f4a455fc08843",
  "time": 1760656845,
  "mediantime": 1760656844,
  "nonce": 3,
  "bits": "207fffff",
  "target": "7fffff0000000000000000000000000000000000000000000000000000000000",
  "difficulty": 4.656542373906925e-10,
  "chainwork": "00000000000000000000000000000000000000000000000000000000000000cc",
  "nTx": 1,
  "previousblockhash": "41bf4350eda2199c9a74d83e0b60dc09ef87eb736e57c443f5c7e93a79d411d7",
  "strippedsize": 213,
  "size": 249,
  "weight": 888,
  "tx": [
    "6cf4ebb6136507dc722dc8afb584bc1370411b89f9fb5119082f4a455fc08843"
  ]
}
```

### Block Structure Analysis

| Component | Value | Significance |
|-----------|-------|--------------|
| **Height** | 101 | Position in blockchain |
| **Merkle Root** | 6cf4ebb6... | Hash of all transactions in block |
| **Previous Hash** | 41bf4350... | Links to parent block (chain integrity) |
| **Nonce** | 3 | PoW solution (Nonce field unused on regtest; often constant or 0) |
| **Transaction Count** | 1 | Single coinbase transaction |
| **Size** | 249 bytes | Including SegWit witness data |
| **Stripped Size** | 213 bytes | Without witness data |
| **Weight** | 888 WU | SegWit weight units (4 * stripped + witness) |

### Technical Insights
- **Merkle Root:** Single transaction means merkle root equals transaction ID
- **Block Version:** `0x20000000` indicates `Version Bits` being used (base BIP9 format), but not signaling for any upgrade in particular, i.e., no signalling of soft-forks/upgrade (no actual _live deployments_ in regtest though)
- **Minimal Nonce:** Low difficulty allows quick mining with small nonce values
- **Weight Calculation:** (213 × 4) + (249 - 213) = 888 weight units

---

## 4. Wallet Management and Address Generation

### Objective
Demonstrate proficiency in creating and managing Bitcoin wallets and generating receiving addresses.

### 4.1 Test Wallet Creation

**Command:**
```bash
bitcoin-cli -regtest createwallet "testwallet"
```

**Output:**
```json
{
  "name": "testwallet"
}
```

### 4.2 Address Generation

Generated multiple native SegWit (bech32) addresses:

**Commands:**
```bash
bitcoin-cli -regtest -rpcwallet=testwallet getnewaddress
```

**Generated Addresses:**
1. `bcrt1q85end92xe7wrye92xhwzfawqz8xlvcx0pt5dn2`
2. `bcrt1qusjaqv2z9tke0sajq0wq8w82qu3nqzeggr0avr`
3. `bcrt1qpz8ryej8cu3amhhhnmmzlzgu9pyv8ja3lwf54e`

### 4.3 Address Listing and Verification

**Command:**
```bash
bitcoin-cli -regtest -rpcwallet=testwallet listreceivedbyaddress 0 true
```

**Output:**
```json
[
  {
    "address": "bcrt1qpz8ryej8cu3amhhhnmmzlzgu9pyv8ja3lwf54e",
    "amount": 0.00000000,
    "confirmations": 0,
    "label": "",
    "txids": []
  },
  {
    "address": "bcrt1q85end92xe7wrye92xhwzfawqz8xlvcx0pt5dn2",
    "amount": 0.00000000,
    "confirmations": 0,
    "label": "",
    "txids": []
  },
  {
    "address": "bcrt1qusjaqv2z9tke0sajq0wq8w82qu3nqzeggr0avr",
    "amount": 0.00000000,
    "confirmations": 0,
    "label": "",
    "txids": []
  }
]
```

### Address Type Analysis
- **Prefix:** `bcrt1q` indicates native SegWit (bech32) on regtest
- **Format:** Witness version 0 (P2WPKH - Pay to Witness Public Key Hash)
- **Benefits:** Lower transaction fees, better error detection, future-proof
- **Compatibility:** Requires modern Bitcoin software support

---

## 5. Transaction Creation and Tracking

### Objective
Execute and monitor Bitcoin transactions, understanding the transaction lifecycle.

### 5.1 Initial Wallet State

Discovered existing wallet with mature coinbase reward:

**Command:**
```bash
bitcoin-cli -regtest -rpcwallet=wallet01 getbalance
```

**Output:** `50.00000000 BTC`

### 5.2 Transaction Execution

**Command:**
```bash
bitcoin-cli -regtest -rpcwallet=wallet01 -named sendtoaddress \
  address=bcrt1q85end92xe7wrye92xhwzfawqz8xlvcx0pt5dn2 \
  amount=5.0 \
  fee_rate=1
```

**Transaction ID:** `71a6cc26f4ec170245eb2e3e4536cee6bb416e6abd51939cbd29109e511d2aab`

### 5.3 Transaction Details (Pre-Confirmation)

**Command:**
```bash
bitcoin-cli -regtest -rpcwallet=wallet01 gettransaction 71a6cc26f4ec170245eb2e3e4536cee6bb416e6abd51939cbd29109e511d2aab
```

**Output:**
```json
{
  "amount": -5.00000000,
  "fee": -0.00000141,
  "confirmations": 0,
  "trusted": true,
  "txid": "71a6cc26f4ec170245eb2e3e4536cee6bb416e6abd51939cbd29109e511d2aab",
  "wtxid": "105ab147301aa6254717f6262462a9b2da888e7d678d74b7fd0fe6b48d74ba37",
  "walletconflicts": [],
  "mempoolconflicts": [],
  "time": 1760979168,
  "timereceived": 1760979168,
  "bip125-replaceable": "yes",
  "details": [
    {
      "address": "bcrt1q85end92xe7wrye92xhwzfawqz8xlvcx0pt5dn2",
      "category": "send",
      "amount": -5.00000000,
      "vout": 1,
      "fee": -0.00000141,
      "abandoned": false
    }
  ]
}
```

### Transaction Analysis
- **Amount Sent:** 5.00000000 BTC
- **Transaction Fee:** 0.00000141 BTC (141 satoshis)
- **Fee Rate:** 1 sat/vB (as specified)
- **Replace-by-Fee:** Enabled (BIP125)
- **Initial State:** Unconfirmed (0 confirmations)
- **TXID vs WTXID:** Different values indicate SegWit transaction

### 5.4 Block Confirmation

**Command:**
```bash
bitcoin-cli -regtest -rpcwallet=wallet01 -generate 1
```

**Output:**
```json
{
  "address": "bcrt1qjyen5sd8wq4w3gvqcp7kr8s6wp4ysf0u5yd7j3",
  "blocks": ["2bd2b730309bdd6cade6c344729a1c7cb857eea4beb6524f9344d36d5bee3757"]
}
```

---

## 6. UTXO Analysis and Structure

### Objective
Understand Bitcoin's UTXO model and how spendable outputs are represented and managed.

### 6.1 Post-Transaction UTXO State

#### Recipient Wallet (testwallet)

**Command:**
```bash
bitcoin-cli -regtest -rpcwallet=testwallet listunspent
```

**Output:**
```json
[
  {
    "txid": "71a6cc26f4ec170245eb2e3e4536cee6bb416e6abd51939cbd29109e511d2aab",
    "vout": 1,
    "address": "bcrt1q85end92xe7wrye92xhwzfawqz8xlvcx0pt5dn2",
    "label": "",
    "scriptPubKey": "00143d33369546cf9c3264aa35dc24f5c011cdf660cf",
    "amount": 5.00000000,
    "confirmations": 1,
    "spendable": true,
    "solvable": true,
    "desc": "wpkh([97396d38/84h/1h/0h/0/0]03f67abe939e5684dc4fd9d192c5988b171e8305072baa2f72c8fc675ee64a9c7e)#lhnsm20e",
    "parent_descs": ["wpkh([97396d38/84h/1h/0h]tpubDC...)#xusjw3wd"],
    "safe": true
  }
]
```

#### Sender Wallet (wallet01)

**Command:**
```bash
bitcoin-cli -regtest -rpcwallet=wallet01 listunspent
```

**Output (Excerpt):**
```json
[
  {
    "txid": "71a6cc26f4ec170245eb2e3e4536cee6bb416e6abd51939cbd29109e511d2aab",
    "vout": 0,
    "address": "bcrt1qxt6ljxmm9wwkmu7t9a330sjyufywkzsuv4dveg",
    "scriptPubKey": "001432f5f91b7b2b9d6df3cb2f6317c244e248eb0a1c",
    "amount": 44.99999859,
    "confirmations": 1,
    "spendable": true,
    "solvable": true,
    "desc": "wpkh([7f90fc03/84h/1h/0h/1/0]0234bc723b9181005f0b99ead473a5ebbb54601bbe4f024cd9b15846d4023aef01)#dzwqueu2",
    "safe": true
  }
]
```

### UTXO Model Breakdown

| Field | Description | Example Value |
|-------|-------------|---------------|
| **txid** | Transaction ID creating this output | 71a6cc26f4ec... |
| **vout** | Output index in transaction | 0 or 1 |
| **address** | Receiving address | bcrt1q85end... |
| **scriptPubKey** | Locking script (hex) | 00143d333695... |
| **amount** | Bitcoin value | 5.00000000 |
| **confirmations** | Blocks since creation | 1 |
| **spendable** | Wallet can spend | true |
| **solvable** | Wallet can create signature | true |
| **desc** | Output descriptor | wpkh([...]) |
| **safe** | No double-spend conflicts | true |

### UTXO Lifecycle Observations

1. **UTXO Creation:**
   - Transaction creates new outputs
   - Each output becomes a distinct UTXO
   - Original input UTXO is consumed (destroyed)

2. **Change Output:**
   - Sender received 44.99999859 BTC as change
   - Original 50 BTC UTXO was split into:
     - 5.0 BTC to recipient
     - ~45 BTC back as change
     - 0.00000141 BTC as fee (destroyed)

3. **Output Descriptor:**
   - Shows HD wallet derivation path
   - Format: `wpkh([fingerprint/derivation]pubkey)`
   - Enables deterministic wallet recovery

---

## 7. Raw Transaction Decoding

### Objective
Deep technical understanding of Bitcoin transaction structure at the protocol level.

### 7.1 Raw Transaction Retrieval

**Command:**
```bash
bitcoin-cli -regtest getrawtransaction 71a6cc26f4ec170245eb2e3e4536cee6bb416e6abd51939cbd29109e511d2aab true
```

### 7.2 Decoded Transaction Structure

```json
{
  "txid": "71a6cc26f4ec170245eb2e3e4536cee6bb416e6abd51939cbd29109e511d2aab",
  "hash": "105ab147301aa6254717f6262462a9b2da888e7d678d74b7fd0fe6b48d74ba37",
  "version": 2,
  "size": 222,
  "vsize": 141,
  "weight": 561,
  "locktime": 0,
  "vin": [
    {
      "txid": "aa09d1ca5616d95adc82456d2c5c3673d92199c78d52367e3c6cd38ee8852297",
      "vout": 0,
      "scriptSig": {
        "asm": "",
        "hex": ""
      },
      "txinwitness": [
        "30440220385c44a73bafbd3252315a4e4854fd8ee11c3bbb6de2eb63445a86c5258cab620220232702a312d08f09bd2ad81d4fc9bf3d94f9c17f520110601848188a696bfdba01",
        "02acd8a19970b8ba003c201d5a7dbeaba962aa62e3e8ceb4988d8b8bc194f966c8"
      ],
      "sequence": 4294967293
    }
  ],
  "vout": [
    {
      "value": 44.99999859,
      "n": 0,
      "scriptPubKey": {
        "asm": "0 32f5f91b7b2b9d6df3cb2f6317c244e248eb0a1c",
        "desc": "addr(bcrt1qxt6ljxmm9wwkmu7t9a330sjyufywkzsuv4dveg)#3tnv5vat",
        "hex": "001432f5f91b7b2b9d6df3cb2f6317c244e248eb0a1c",
        "address": "bcrt1qxt6ljxmm9wwkmu7t9a330sjyufywkzsuv4dveg",
        "type": "witness_v0_keyhash"
      }
    },
    {
      "value": 5.00000000,
      "n": 1,
      "scriptPubKey": {
        "asm": "0 3d33369546cf9c3264aa35dc24f5c011cdf660cf",
        "desc": "addr(bcrt1q85end92xe7wrye92xhwzfawqz8xlvcx0pt5dn2)#nm5tsj4v",
        "hex": "00143d33369546cf9c3264aa35dc24f5c011cdf660cf",
        "address": "bcrt1q85end92xe7wrye92xhwzfawqz8xlvcx0pt5dn2",
        "type": "witness_v0_keyhash"
      }
    }
  ]
}
```

### 7.3 Transaction Anatomy

#### Transaction Metadata
- **Version:** 2 (enables BIP68 relative time locks)
- **Size:** 222 bytes (total transaction size)
- **Virtual Size (vSize):** 141 vBytes (used for fee calculation)
- **Weight:** 561 weight units (SegWit metric)
- **Locktime:** 0 (transaction can be included immediately)

#### Input Analysis (vin)
- **Previous TXID:** aa09d1ca5616d95adc82456d2c5c3673d92199c78d52367e3c6cd38ee8852297
- **Previous vout:** 0 (spending first output of previous transaction)
- **scriptSig:** Empty (SegWit transaction moves signatures to witness)
- **txinwitness:** Contains:
  1. DER-encoded ECDSA signature (71 bytes)
  2. Compressed public key (33 bytes)
- **Sequence:** 4294967293 (0xFFFFFFFD) - enables RBF and relative time locks

#### Output Analysis (vout)

**Output 0 - Change:**
- Value: 44.99999859 BTC
- Type: witness_v0_keyhash (P2WPKH)
- Address: bcrt1qxt6ljxmm9wwkmu7t9a330sjyufywkzsuv4dveg
- scriptPubKey: OP_0 + 20-byte pubkey hash

**Output 1 - Payment:**
- Value: 5.00000000 BTC
- Type: witness_v0_keyhash (P2WPKH)
- Address: bcrt1q85end92xe7wrye92xhwzfawqz8xlvcx0pt5dn2
- scriptPubKey: OP_0 + 20-byte pubkey hash

### 7.4 SegWit Benefits Demonstrated

1. **Transaction Malleability Fix:**
   - TXID excludes witness data
   - WTXID includes witness data
   - Prevents third-party transaction ID modification

2. **Fee Efficiency:**
   - Virtual size (141 vB) < actual size (222 bytes)
   - Witness data receives 75% discount
   - Calculation: (base_size × 3 + total_size) / 4

3. **Block Capacity:**
   - Weight units allow more transactions per block
   - Maximum block weight: 4,000,000 WU
   - Effective block size increase without hard fork

### 7.5 Transaction Verification

**Input Value:** 50.00000000 BTC (from previous UTXO)
**Output Values:**
- Change: 44.99999859 BTC
- Payment: 5.00000000 BTC
- **Total Outputs:** 49.99999859 BTC

**Transaction Fee:**
50.00000000 - 49.99999859 = **0.00000141 BTC (141 satoshis)**

**Fee Rate Verification:**
141 satoshis / 141 vBytes = **1 sat/vB** ✓

---

## 8. Payment Workflow Simulation

### Objective
Demonstrate complete understanding of Bitcoin payment lifecycle from wallet creation through transaction confirmation.

### 8.1 Wallet Creation

**Commands:**
```bash
bitcoin-cli -regtest createwallet "sender"
bitcoin-cli -regtest createwallet "receiver"
```

**Success Output:**
```json
{ "name": "sender" }
{ "name": "receiver" }
```

### 8.2 Address Generation

**Sender Address:**
```bash
bitcoin-cli -regtest -rpcwallet=sender getnewaddress
# Output: bcrt1qmjv4x6kjhr7cwjz34a3yyy26fsf06tld3ld3sq
```

**Receiver Address:**
```bash
bitcoin-cli -regtest -rpcwallet=receiver getnewaddress
# Output: bcrt1qqw3uexu5cr90assud4929h9tfw9553fr6mnluf
```

### 8.3 Funding Sender Wallet

Generated 101 blocks to sender wallet to create mature coinbase rewards:

**Command:**
```bash
bitcoin-cli -regtest -rpcwallet=sender -generate 101
```

**Result:**
- Blocks generated: 101
- Sender balance: 50.00000000 BTC (first block's coinbase reward)

### 8.4 Payment Execution

**Command:**
```bash
bitcoin-cli -regtest -rpcwallet=sender -named sendtoaddress \
  address=bcrt1qqw3uexu5cr90assud4929h9tfw9553fr6mnluf \
  amount=10.0 \
  fee_rate=1
```

**Transaction ID:** `f983ddf76c66e7576f6af9718fc2204575eb0d4d803d5ce69ccd3c91f85a07f3`

### 8.5 Transaction Details

**Command:**
```bash
bitcoin-cli -regtest getrawtransaction f983ddf76c66e7576f6af9718fc2204575eb0d4d803d5ce69ccd3c91f85a07f3 true
```

**Key Details:**
```json
{
  "txid": "f983ddf76c66e7576f6af9718fc2204575eb0d4d803d5ce69ccd3c91f85a07f3",
  "version": 2,
  "size": 222,
  "vsize": 141,
  "weight": 561,
  "locktime": 203,
  "vin": [
    {
      "txid": "0a69039036c2bc36b232ba54e483fe785f91f1705ae60a948baba201dd432a36",
      "vout": 0
    }
  ],
  "vout": [
    {
      "value": 10.00000000,
      "n": 0,
      "scriptPubKey": {
        "address": "bcrt1qqw3uexu5cr90assud4929h9tfw9553fr6mnluf",
        "type": "witness_v0_keyhash"
      }
    },
    {
      "value": 39.99999859,
      "n": 1,
      "scriptPubKey": {
        "address": "bcrt1qyv7prg4y6vadl36j5g6hpewx3jr8mg4r2mmejj",
        "type": "witness_v0_keyhash"
      }
    }
  ]
}
```

### 8.6 Transaction Confirmation

**Command:**
```bash
bitcoin-cli -regtest -rpcwallet=sender -generate 1
```

**Confirmation Block:**
```json
{
  "address": "bcrt1qstsdj4amrsfl0r8phufgd56g000lfynpd25asa",
  "blocks": ["04c3f22cba9d26c3bc24dbcb856f58d7c181124b96e5d4075a485dce1818c37a"]
}
```

### 8.7 Final Balance Verification

**Receiver Balance:**
```bash
bitcoin-cli -regtest -rpcwallet=receiver getbalance
# Output: 10.00000000
```

**Sender Balance:**
```bash
bitcoin-cli -regtest -rpcwallet=sender getbalance
# Output: 89.99999859
```

### 8.8 Balance Reconciliation

**Initial State:**
- Sender: 50.00000000 BTC (from coinbase)
- Receiver: 0.00000000 BTC

**After Payment:**
- Sender: 89.99999859 BTC
  - Original: 50.00000000
  - Mining reward: 50.00000000 (block 102)
  - Sent: -10.00000000
  - Fee: -0.00000141
- Receiver: 10.00000000 BTC

**Transaction Accounting:**
```
Sender Final = Initial + Mining Reward - Amount Sent - Fee
89.99999859 = 50.00000000 + 50.00000000 - 10.00000000 - 0.00000141 ✓
```

### 8.9 Mempool Status

**Command:**
```bash
bitcoin-cli -regtest getmempoolinfo
```

**Output:**
```json
{
  "loaded": true,
  "size": 0,
  "bytes": 0,
  "usage": 64,
  "total_fee": 0.00000000,
  "maxmempool": 300000000,
  "mempoolminfee": 0.00000100,
  "minrelaytxfee": 0.00000100,
  "incrementalrelayfee": 0.00000100,
  "unbroadcastcount": 0,
  "fullrbf": true,
  "permitbaremultisig": true,
  "maxdatacarriersize": 100000
}
```

**Analysis:**
- Mempool is empty (all transactions confirmed)
- Minimum relay fee: 1 sat/vB
- Full RBF enabled (BIP125 enforcement)

---

## 9. Key Learnings and Observations

### 9.1 UTXO Model vs Account Model

**Understanding:**
- Bitcoin uses UTXO (Unspent Transaction Output) model, not account balances
- Each transaction consumes existing UTXOs and creates new ones
- Wallet "balance" is the sum of all spendable UTXOs
- No concept of "updating" a balance - only creating and destroying outputs

**Advantages:**
- Better privacy (no global account state)
- Parallel transaction processing
- Clear ownership semantics
- Simpler verification

### 9.2 Coinbase Maturity

**Rule:** Newly mined coins require 100 confirmations before spending

**Purpose:**
- Protects against chain reorganizations
- Prevents miners from double-spending coinbase rewards
- Ensures blockchain stability

**Observed:** Generated 101 blocks to make first coinbase spendable

### 9.3 Transaction Fees

**Fee Market Mechanics:**
- Fee = Input Value - Output Value
- Fee rate measured in satoshis per virtual byte (sat/vB)
- Miners prioritize higher fee rate transactions
- In regtest: Used 1 sat/vB (minimum rate)

**SegWit Impact:**
- Witness data receives 75% weight discount
- Reduces effective transaction cost
- On my transaction: 222 bytes actual, 141 vB effective

### 9.4 SegWit (Segregated Witness)

**Technical Benefits Demonstrated:**
1. **Transaction Malleability Fix:** Separate witness data prevents TXID manipulation
2. **Block Capacity Increase:** Weight units allow more transactions
3. **Fee Efficiency:** Witness discount reduces costs
4. **Script Upgrades:** Enables future protocol improvements
5. **Lightning Network:** Foundation for second-layer scaling

**Address Format:**
- Native SegWit: `bcrt1q...` (bech32)
- Version byte: 0 (P2WPKH)
- Future-proof: Supports version upgrades

### 9.5 Transaction Lifecycle

**Complete Flow Observed:**
1. **Creation:** Wallet constructs transaction spending UTXOs
2. **Signing:** Private keys sign transaction inputs
3. **Broadcasting:** Transaction enters mempool
4. **Mempool:** Awaits miner selection
5. **Confirmation:** Mined into block
6. **Finality:** Additional confirmations increase security

**State Transitions:**
- Unconfirmed → 1 confirmation → 6+ confirmations (standard finality)
- In regtest: Instant mining, but demonstrates same principles

### 9.6 HD Wallets (Hierarchical Deterministic)

**Observed in Output Descriptors:**
```
wpkh([97396d38/84h/1h/0h/0/0]03f67abe...)
```

**Structure:**
- Master fingerprint: 97396d38
- Derivation path: m/84'/1'/0'/0/0
  - 84': BIP84 (native SegWit)
  - 1': Testnet/Regtest coin type
  - 0': Account 0
  - 0: External chain (receiving addresses)
  - 0: Address index

**Benefits:**
- Single seed generates infinite addresses
- Backup simplicity (seed phrase)
- Privacy through address reuse avoidance
- Deterministic recovery

### 9.7 Replace-by-Fee (RBF - BIP125)

**Observed:** `"bip125-replaceable": "yes"`

**Mechanism:**
- Sequence number < 0xFFFFFFFE signals RBF
- Allows fee bumping for stuck transactions
- Sender can replace unconfirmed transaction with higher fee version

**My transaction:** Sequence 4294967293 (0xFFFFFFFD) enables RBF

---

## 10. Technical Insights

### 10.1 Regtest vs Other Networks

| Aspect | Regtest | Testnet | Mainnet |
|--------|---------|---------|---------|
| **Mining** | Instant (on-demand) | ~10 min average | ~10 min average |
| **Difficulty** | Minimal | Dynamic | Dynamic |
| **Coins** | No value | No value | Real value |
| **Network** | Local | Public | Public |
| **Reset** | Anytime | Never | Never |
| **Use Case** | Development | Testing | Production |

### 10.2 Block Structure Deep Dive

**Components:**
1. **Block Header (80 bytes):**
   - Version (4 bytes)
   - Previous block hash (32 bytes)
   - Merkle root (32 bytes)
   - Timestamp (4 bytes)
   - Difficulty target (4 bytes)
   - Nonce (4 bytes)

2. **Transaction Count:** Variable-length integer

3. **Transactions:** Variable-length array

**Block 101:**
- Header hash: 3c2f3cdf8cd7b7754499eb52fdefaf4203b63788c50d4cc9bc0d310526738d35
- Single transaction: Coinbase
- Merkle root equals transaction ID (single tx optimization)

### 10.3 Script Types Encountered

**P2WPKH (Pay to Witness Public Key Hash):**
- ScriptPubKey: `OP_0 <20-byte-hash>`
- Witness: `<signature> <pubkey>`
- Native SegWit format
- Most efficient standard script

**Hex Example:**
```
00143d33369546cf9c3264aa35dc24f5c011cdf660cf
│  └─────────────────┬─────────────────────┘
│                     └─ 20-byte pubkey hash
└─ OP_0 (witness version 0)
```

### 10.4 Transaction Verification Process

**Steps Performed by Nodes:**
1. **Structure Validation:** Correct format and fields
2. **Input Verification:** Referenced UTXOs exist and unspent
3. **Script Execution:** Unlocking scripts satisfy locking scripts
4. **Signature Validation:** ECDSA signatures verify correctly
5. **Value Conservation:** Inputs ≥ Outputs (difference = fee)
6. **Consensus Rules:** All protocol rules satisfied

**Transaction Passed All Checks:**
- Valid input reference
- Correct signatures in witness
- Proper value accounting
- Minimum fee paid

### 10.5 Merkle Tree Concept

**Purpose:**
- Efficiently prove transaction inclusion in block
- Enable SPV (Simplified Payment Verification)
- Constant-size proof (log n hashes)

**On Block:**
- Single transaction: Merkle root = Transaction hash
- Multi-transaction blocks: Binary tree of hashes
- Root included in block header

### 10.6 Bitcoin Address Encoding

**Bech32 Format (Native SegWit):**
- Human-readable part: "bcrt" (regtest)
- Separator: "1"
- Data part: Witness version + program
- Checksum: 6 characters

**Example:** `bcrt1q85end92xe7wrye92xhwzfawqz8xlvcx0pt5dn2`
- Prefix: bcrt (regtest)
- Version: 1q → 0 (witness v0)
- Program: 20-byte pubkey hash
- Checksum: Built-in error detection

### 10.7 Network Final State

**Final Block Count:** 204 blocks

**Breakdown:**
- Initial: 101 blocks
- Sender mining: +101 blocks
- Confirmation blocks: +2 blocks

**Wallets Created:**
1. wallet01 (pre-existing)
2. testwallet
3. sender
4. receiver

**Successful Transactions:**
1. wallet01 → testwallet: 5 BTC
2. sender → receiver: 10 BTC

---

## Conclusion

This assignment successfully demonstrated comprehensive understanding of Bitcoin fundamentals through practical hands-on experience. Key accomplishments include:


### Protocol Understanding
- **UTXO Model:** Deep understanding of unspent output management
- **Transaction Structure:** Complete knowledge of inputs, outputs, witnesses
- **SegWit Benefits:** Transaction malleability fix, fee efficiency, capacity increase
- **Protocol upgrade/activation frameworks:** BIP9 provides a standard signalling mechanism for deploying soft-fork consensus changes in the Bitcoin protocol, while BIP8 improve upon BIP 9 by addressing some of its perceived drawbacks
- **Fee Markets:** Proper fee rate calculation and mempool dynamics
- **Consensus Rules:** Coinbase maturity, script validation, value conservation
- **HD Wallets:** BIP32/BIP44/BIP84 derivation paths and deterministic generation

---

## Appendix: Command Reference

### Node Operations
```bash
bitcoin-cli -regtest getblockchaininfo
bitcoin-cli -regtest getblockcount
bitcoin-cli -regtest getbestblockhash
bitcoin-cli -regtest getblock <blockhash>
bitcoin-cli -regtest getblockheader <blockhash>
bitcoin-cli -regtest getmempoolinfo
```

### Wallet Management
```bash
bitcoin-cli -regtest createwallet "<name>"
bitcoin-cli -regtest listwallets
bitcoin-cli -regtest -rpcwallet=<name> getnewaddress
bitcoin-cli -regtest -rpcwallet=<name> getbalance
bitcoin-cli -regtest -rpcwallet=<name> listunspent
bitcoin-cli -regtest -rpcwallet=<name> listreceivedbyaddress 0 true
```

### Transaction Operations
```bash
bitcoin-cli -regtest -rpcwallet=<name> -named sendtoaddress address=<addr> amount=<btc> fee_rate=<sat/vb>
bitcoin-cli -regtest -rpcwallet=<name> gettransaction <txid>
bitcoin-cli -regtest getrawtransaction <txid> true
```

### Mining (Regtest)
```bash
bitcoin-cli -regtest -rpcwallet=<name> -generate <blocks>
bitcoin-cli -regtest generatetoaddress <blocks> "<address>"
```

---
