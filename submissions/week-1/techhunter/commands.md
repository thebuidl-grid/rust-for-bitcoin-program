1. `bitcoind -regtest -daemon` for starting my bitcoin core

2. `bitcoin-cli -regtest getblockchaininfo` to get blockchain info
3. creating a wallet `bitcoin-cli -regtest createwallet "mywallet"`
4. mined block 101 `bitcoin-cli -regtest generatetoaddress 101 "$(bitcoin-cli -regtest -rpcwallet=mywallet getnewaddress)"`
5. get my block count `bitcoin-cli -regtest getblockcount`
6. Getting a new address to send to `NEW_ADDRESS=$(bitcoin-cli -regtest -rpcwallet=mywallet getnewaddress)`
7. sent 10 BTC to the address `TXID=$(bitcoin-cli -regtest -rpcwallet=mywallet sendtoaddress $NEW_ADDRESS 10)`
8. Retrieve transaction details `bitcoin-cli -regtest -rpcwallet=mywallet gettransaction $TXID`
9. Mine a block to comfirm transaction `bitcoin-cli -regtest generatetoaddress 1 "$(bitcoin-cli -regtest -rpcwallet=mywallet getnewaddress)"`
10. Comfirm the transaction is comfirmed `bitcoin-cli -regtest -rpcwallet=mywallet gettransaction $TXID`
11. List all unspent transaction outputs
    `bitcoin-cli -regtest -rpcwallet=mywallet listunspent`

12. List only UTXOs with at least 6 confirmations
    `bitcoin-cli -regtest -rpcwallet=mywallet listunspent 6`![alt text](<Screenshot 2025-11-12 at 12.42.01.png>)
13. Get a transaction ID from previous tasks
    `TXID=$(bitcoin-cli -regtest -rpcwallet=mywallet listunspent | jq -r '.[0].txid')`

14. Get raw transaction (decoded)
    `bitcoin-cli -regtest getrawtransaction $TXID true`

15. Alternative: Get raw hex and decode separately
    `RAW_TX=$(bitcoin-cli -regtest getrawtransaction $TXID)`
    `bitcoin-cli -regtest decoderawtransaction $RAW_TX`

16. Create sender wallet
    `bitcoin-cli -regtest createwallet "sender"`

17. Create receiver wallet
    `bitcoin-cli -regtest createwallet "receiver"`

18. Fund sender wallet (mine 101 blocks to sender)
    `SENDER_ADDRESS=$(bitcoin-cli -regtest -rpcwallet=sender getnewaddress)`
    `bitcoin-cli -regtest generatetoaddress 101 $SENDER_ADDRESS`

19. Check sender balance
    `bitcoin-cli -regtest -rpcwallet=sender getbalance`

20. Get receiver address
    `RECEIVER_ADDRESS=$(bitcoin-cli -regtest -rpcwallet=receiver getnewaddress)`

21. Send 25 BTC from sender to receiver
    `PAYMENT_TXID=$(bitcoin-cli -regtest -rpcwallet=sender sendtoaddress $RECEIVER_ADDRESS 25)`

22. Check transaction (unconfirmed)
    `bitcoin-cli -regtest -rpcwallet=sender gettransaction $PAYMENT_TXID`

23. Mine a block to confirm
    `bitcoin-cli -regtest generatetoaddress 1 $SENDER_ADDRESS`

24. Verify receiver got the payment
    `bitcoin-cli -regtest -rpcwallet=receiver getbalance`
    `bitcoin-cli -regtest -rpcwallet=receiver listtransactions`
