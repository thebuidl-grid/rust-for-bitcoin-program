### 1. Verify Your Node Setup: Ensure your node is correctly configured and responsive via CLI

![Screenshot](https://res.cloudinary.com/dad1drjht/image/upload/v1760935877/Screenshot_2025-10-20_at_5.48.02_AM_rkxejc.png)

- chain: regtest
- blocks: 105

### 2. Generate Blocks: Understand block creation and coinbase maturity on regtest

![Screenshot](https://res.cloudinary.com/dad1drjht/image/upload/v1760936398/Screenshot_2025-10-20_at_5.58.29_AM_v0ewuy.png)

- block count: 206

### 3. Explore the Blockchain: Learn the structure and metadata of Bitcoin blocks

![screenshot](https://res.cloudinary.com/dad1drjht/image/upload/v1760937035/Screenshot_2025-10-20_at_6.10.02_AM_i5fia0.png)

### 4. Work with Wallets: Get comfortable managing multiple wallets and addresses

![screenshot](https://res.cloudinary.com/dad1drjht/image/upload/v1760937729/Screenshot_2025-10-20_at_6.20.41_AM_hcxw9p.png)

- "testwallet" already exist, so I created few new addresses

### 5. Send and Track Transactions: Learn how transactions are created, broadcast, and confirmed

![screenshot](https://res.cloudinary.com/dad1drjht/image/upload/v1760940428/Screenshot_2025-10-20_at_7.05.45_AM_chck1p.png)

- I had to specify the fee_rate for the txn to go

### 6. Inspect UTXOs: Send and Track Transactions: Understand how UTXOs represent spendable Bitcoin

![screenshot](https://res.cloudinary.com/dad1drjht/image/upload/v1760940866/Screenshot_2025-10-20_at_7.12.43_AM_tug5zf.png)

- txid: 631515746bdb98691d8897d38a6f55572a4ba3ffe03c40628d30a610be3e94ef
- vout: 0
- amount: 50.00000000

### 7. Decode a Raw Transaction: Deepen understanding of Bitcoin’s transaction model

![screenshot](https://res.cloudinary.com/dad1drjht/image/upload/v1760941343/Screenshot_2025-10-20_at_7.20.48_AM_mekj7u.png)

### 8. Simulate a Payment Workflow: Experience the full transaction lifecycle in regtest

#### Commands Used

- `bitcoin-cli createwallet "sender"`

```zsh
bitcoin-cli createwallet "sender"
{
  "name": "sender"
}
```

- `bitcoin-cli createwallet "receiver"`

```zsh
bitcoin-cli createwallet "receiver"
{
  "name": "receiver"
}
```

- `bitcoin-cli -rpcwallet=sender getnewaddress`

```zsh
bitcoin-cli -rpcwallet=sender getnewaddress
bcrt1q7c5gvnmj9ay27nzh6669htsspvexl2qkf7k2z6
```

- `bitcoin-cli -rpcwallet=sender generatetoaddress 101 bcrt1q7c5gvnmj9ay27nzh6669htsspvexl2qkf7k2z6`

```zsh
bitcoin-cli -rpcwallet=sender generatetoaddress 101 bcrt1q7c5gvnmj9ay27nzh6669htsspvexl2qkf7k2z6
[
  "7dd5e537d502d7eb0550594bab53467b5db837cdb0700ecf0ae53bddecf45e89",
  "197325026cc761d970243a4fc15996457d22eb8b8456dcc5761534912e734258",
  "5130b4dd88a2d198b8b193184af2aefa2c42f943f7bd77785cf48935ef6a28d7",
  ...
]
```

- `bitcoin-cli -rpcwallet=sender getbalance`

```zsh
bitcoin-cli -rpcwallet=sender getbalance
25.00000141
```

**I defined a low fallbackfee that's why it's 25.00000141**

- `bitcoin-cli -rpcwallet=receiver getnewaddress`
  
```zsh
bitcoin-cli -rpcwallet=receiver getnewaddress
bcrt1qljthxv30lm962v92jht7yxw4324kr4z0mz9m7h
```

- `bitcoin-cli -rpcwallet=sender sendtoaddress bcrt1qljthxv30lm962v92jht7yxw4324kr4z0mz9m7h 5 "" "" false false null "unset" null 1`

```zsh
bitcoin-cli -rpcwallet=sender sendtoaddress bcrt1qljthxv30lm962v92jht7yxw4324kr4z0mz9m7h 5 "" "" false false null "unset" null 1
611f13bb2f3adee8ba93d76e465aacc600f7c25e718c8c0d05169defbb898043
```

- `bitcoin-cli -rpcwallet=sender generatetoaddress 1 bcrt1q7c5gvnmj9ay27nzh6669htsspvexl2qkf7k2z6`

```zsh
bitcoin-cli -rpcwallet=sender generatetoaddress 1 bcrt1q7c5gvnmj9ay27nzh6669htsspvexl2qkf7k2z6
[
  "498a91e61d60807388402a951df884ad95bcc6a7cff33c2cb76b71753cbc76d7"
]
```

- `bitcoin-cli -rpcwallet=receiver getbalance`

```zsh
bitcoin-cli -rpcwallet=receiver getbalance
5.00000000
```

- `bitcoin-cli -rpcwallet=receiver gettransaction 611f13bb2f3adee8ba93d76e465aacc600f7c25e718c8c0d05169defbb898043`
- `bitcoin-cli -rpcwallet=sender gettransaction 611f13bb2f3adee8ba93d76e465aacc600f7c25e718c8c0d05169defbb898043`

![screenshot](https://res.cloudinary.com/dad1drjht/image/upload/v1760943236/Screenshot_2025-10-20_at_7.53.36_AM_asini0.png)
