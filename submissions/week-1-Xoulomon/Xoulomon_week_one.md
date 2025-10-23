## Week 1 Assignment Solution.

1. ## successfuly setup my node and run getblockinfo.

![Result screenshot](./screenshot//01_getblockinfo.png)

2. ## Create new wallets

![Result screenshot](./screenshot/002_createwallet.png)
   
   ### List wallet generated

![Result screenshot](./screenshot/003_listwallets.png)

3. ## Generate 101 blocks

![Result screenshot](./screenshot/004_generate_101_blocks.png.png)

   > bitcoin-cli getblockcount:

   ![Result screenshot](./screenshot/005_getblockcount.png)

   > bitcoin-cli getblock <blockhash>

   ![Result screenshot](./screenshot/006_getblock.png)

   > bitcoin-cli getblockhear <blockhash>

   ![Result screenshot](./screenshot/007_getblockheader.png)

4. ## Create set of wallet

   created a new wallets

   ![Result screenshot](./screenshot/008_creatwallet_listwallet.png)

5. ## Get wallet balance, Get new address, Send to address, Get transaction

   ![Result screenshot](./screenshot/009_getbalance_getnewaddress_sendtoaddress_gettransaction.png)

6. ## used command listunspent to view my spendable balance 

    ![Result screenshot](./screenshot/011_listunspent.png)

7. ## fetched transaction with gettransaction
   ![Result screenshot](./screenshot/012_gettransaction.png)

8. ## sent BTC between two wallets
> first i created two wallet named sender and receiver 

![Result screenshot](./screenshot/013_createwallet_sender_reciever_listwallets.png)

> sent BTC from sender to receiver and for the transaction to reflect in receiver's wallet, i have to mine a block.

![Result screenshot](./screenshot/014_getbalance_sendtoaddress_generate_1_block.png)

