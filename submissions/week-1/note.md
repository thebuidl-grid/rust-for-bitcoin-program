# Week 1 Assignment 

I learnt the fundamentals of the bitcoin network and how to setup a local development environment using Bitcoin core.

### 1. Startup deamon and getblockchaininfo:
![alt text](1.png)

### 2. GenrateBlocks:
![alt text](<Screenshot 2025-10-18 121634.png>)

![alt text](<Screenshot 2025-10-18 121933.png>)

![alt text](<Screenshot 2025-10-18 122159.png>)

### 3. Inspect blocks:
![alt text](<Screenshot 2025-10-18 123018.png>)

![alt text](<Screenshot 2025-10-18 123226.png>)

### 4. Multiple wallets and addresses:
![alt text](<Screenshot 2025-10-18 124026.png>)

![alt text](<Screenshot 2025-10-18 135825.png>)

![alt text](<Screenshot 2025-10-18 140349.png>)

### 5. Send and Track Transactions:
![alt text](<Screenshot 2025-10-18 161855.png>)

![alt text](<Screenshot 2025-10-18 162448.png>)

![alt text](<Screenshot 2025-10-18 163409.png>)

### 6. Inspect UTXOs:
![alt text](<Screenshot 2025-10-18 164115.png>)

### 7. Decode a Raw Transaction:
```
./bitcoin-cli getrawtransaction \
 "c9bbefa4d35458542603e7d9fe1b481369a2852e0e5c38a6d06b0753baa27750" \
  1 \
  "1f8dc73d3604981bb2ec987fd9a029015ddbaa7ba36af055f490bf9bcfb7121f"
========================================================================

{
  "in_active_chain": true,
  "txid": "c9bbefa4d35458542603e7d9fe1b481369a2852e0e5c38a6d06b0753baa27750",
  "hash": "0622c3d9d7a4f487a1b55ab019febf931eb5c754663c34f6d6d0c06e9be6c301",
  "version": 2,
  "size": 222,
  "vsize": 141,
  "weight": 561,
  "locktime": 213,
  "vin": [
    {
      "txid": "4db54ae586d5fce1fb17d4c678b9378682f28b9dc3ff41aceed164b5c3b96836",
      "vout": 0,
      "scriptSig": {
        "asm": "",
        "hex": ""
      },
      "txinwitness": [
        "30440220135ae7fea7ab5141253ff76bd766cab8c8d7ab4bed1532039fbe6406866e696c02207fe9d62926eb8dfb11f63568755c4d1c31bfac07c5af4558906389887c4e756201",
        "02a3639498067ca12af9271884a3e43aaf75b7de344954b56d69cbcd1747d07713"
      ],
      "sequence": 4294967293
    }
  ],
  "vout": [
    {
      "value": 44.99999295,
      "n": 0,
      "scriptPubKey": {
        "asm": "0 a48deae14d6821246dbe7717fee9ae3cbc4398c7",
        "desc": "addr(bcrt1q5jx74c2ddqsjgmd7wutla6dw8j7y8xx872k5c0)#l46z9m4a",
        "hex": "0014a48deae14d6821246dbe7717fee9ae3cbc4398c7",
        "address": "bcrt1q5jx74c2ddqsjgmd7wutla6dw8j7y8xx872k5c0",
        "type": "witness_v0_keyhash"
      }
    },
    {
      "value": 5.00000000,
      "n": 1,
      "scriptPubKey": {
        "asm": "0 71157261018106c7f5eb7b59e1ea916a8e1b19c3",
        "desc": "addr(bcrt1qwy2hycgpsyrv0a0t0dv7r653d28pkxwr40cfn8)#780m4jag",
        "hex": "001471157261018106c7f5eb7b59e1ea916a8e1b19c3",
        "address": "bcrt1qwy2hycgpsyrv0a0t0dv7r653d28pkxwr40cfn8",
        "type": "witness_v0_keyhash"
      }
    }
  ],
  "hex": "020000000001013668b9c3b564d1eeac41ffc39d8bf2828637b978c6d417fbe1fcd586e54ab54d0000000000fdffffff023f8a380c01000000160014a48deae14d6821246dbe7717fee9ae3cbc4398c70065cd1d0000000016001471157261018106c7f5eb7b59e1ea916a8e1b19c3024730440220135ae7fea7ab5141253ff76bd766cab8c8d7ab4bed1532039fbe6406866e696c02207fe9d62926eb8dfb11f63568755c4d1c31bfac07c5af4558906389887c4e7562012102a3639498067ca12af9271884a3e43aaf75b7de344954b56d69cbcd1747d07713d5000000",
  "blockhash": "1f8dc73d3604981bb2ec987fd9a029015ddbaa7ba36af055f490bf9bcfb7121f",
  "confirmations": 10,
  "time": 1760801432,
  "blocktime": 1760801432
}
```
### 8. Sendrawtransaction
![alt text](<Screenshot 2025-10-18 183117.png>)

![alt text](<Screenshot 2025-10-18 183624.png>)

![alt text](<Screenshot 2025-10-18 183744.png>)