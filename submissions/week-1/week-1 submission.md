## Assignment solution.

1. ## successfuly setup my node and its correct and responsive.

"chain": "regtest",
"blocks": 102,

![Result screenshot]
(screenshot/001.png)

2. ## I generated 101 new block to new address

   ![Result screenshot]
   (screenshot/002.png)

  > Ran bitcoin-cli getblockcount and got the new block height:

   ![Result screenshot]
   (screenshot/003.png)

3. ## Learnt about the structure and the metadata of bitcoin blocks

   > bitcoin-cli getbestblock:

   ![Result screenshot]
   (screenshot/004.png)

   > bitcoin-cli getblock<blockhash>

   ![Result screenshot]
   (screenshot/005.png)

   > bitcoin-cli getblockhear <blockhash>

   ![Result screenshot]
   (screenshot/006.png)

4. ## i was able to manage mutiple wallet and addresses

   created a new wallet and named it papsBTC:

   ![Result screenshot]
   (screenshot/007.png)

5. ## sent test coint to new address 

   ![Result screenshot]
   (screenshot/008.png)

6. ## used command listunspent to view my spendable balance 

    ![Result scrennshot]
    (screenshot/009.png)
  
  TXID : 2662e498a408534a8a8284331dca2da9eb2a1dbf21c36a8291c0d3ce20f6ce8a"
  VOUT : 1
  AMOUNT : 39.99997180

7. ## fetched transaction with getrawtransaction
   ![Result screenshot]
   (screenshot/010.png)

8. ## sent BTC between two wallets
> first i created two wallet named sender and receiver 
     ![Result screenshot]
     (screenshot/011.png)

> sent BTC from sender to receiver and for the transaction to reflect in receiver's wallet, i have to mine a block.

     ![Result screenshot]
      (screenshot/012.png)


