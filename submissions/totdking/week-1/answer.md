## 1. Verify node setup
- Run bitcoin in regtest mode without daemon `bitcoind -regtest`
- Run bitcoin in regtest mode with daemon `bitcoind -regtest -daemon` : 
![daemon running](./../Screenshot%20from%202025-10-18%2012-22-24.png)

- Run `bitcoin-cli getblockchaininfo`: 
`
{
  "chain": "main",
  "blocks": 182082,
  "headers": 919639,
  "bestblockhash": "00000000000001d9577471b60028b1f2f5a62326bcca90e3871d8ebb9000c2ed",
  "bits": "1a0a8b5f",
  "target": "0000000000000a8b5f0000000000000000000000000000000000000000000000",
  "difficulty": 1591074.961847305,
  "time": 1338287579,
  "mediantime": 1338282275,
  "verificationprogress": 0.00282506340625284,
  "initialblockdownload": true,
  "chainwork": "000000000000000000000000000000000000000000000012519649fafeec8da2",
  "size_on_disk": 1756499381,
  "pruned": false,
  "warnings": [
  ]
}
`


## 2. Generate blocks
- use this command `bitcoin-cli -regtest generatetoaddress 101 "$(bitcoin-cli -regtest getnewaddress)"` to generate 101 blocks to a new address

- first create wallet `bitcoin-cli -regtest createwallet "mywallet"`
- run the generation of 101 blocks to the address
```
$ bitcoin-cli -regtest createwallet "mywallet"
{
  "name": "mywallet"
}
$ bitcoin-cli -regtest generatetoaddress 101 "$(bitcoin-cli -regtest getnewaddress)"
[
  "577d9d077d727850a57c4da4a5699b5cc6713f23a29d051c92ec118b08ecca90",
  "2a96de7dc8a865dedb8981850a9adec7e42b221c3249b20ef4e8c514902391d2",
  "68ee2839c3c59bbb851bd5ca034685e0d543d97cb9a02cc42584d5bcc5c2de7f",
  "3ecac164573ae82fa644aedea2840e1c1e85ebb0ccf84b486630ef4f91253c34",
  "6a975a4f5624fb23d08f7d8712c79f9a4315d26a776cf5ca08328d0d0dd4a78c",
  "347d199806ffde334e8f3665fd7a75e49d86fcbfb823d0fe7245ebeeed63793b",
  "6585f82d89df3066a831bb19834bc533958b208c72efb314991f057f1fa9a86e",
  "742725768d05ce4d8d3d1dc592730a6bd04c7fc7086ce6ea29dbef71b16b4f4d",
  "05bb391c003fd3c9a31aa35d0f9d901f3da48c3be15e7ca9a900f60d6467035c",
  "4bd6a6944076860d3676e757b7cbcdfe9d9226175b999b05d2d685543b96317b",
  "7ebc3d822f1176ff16b4d3c7c38470619c16b386a292a4a4518bf4b775c612a4",
  "4e9eb32f89a43d461f47dc2638ff1e0056ca14514e58449532d6620a6dc3313c",
  "2bf15430712eb0ef5cf65f46c55738cad6c55bc400e30e11cebfefe8169343eb",
  "4e0fabf0a6f4e9d6fbd2ee2b8579cba5e0729490b59136a7ec6f46a5f4ba2f6d",
  "187fd2b155d241522d7b9964b80be3730db3eed9d93cd7e83fd4b2e8bc109d52",
  "091add8edbe58aa48f655da39324f3dba2a700751595acbcd4381cd1876166b2",
  "1a05252d13ec92c4ca6c70c67edabace68f6ded5a383420e947e7c6c45b476f6",
  "494bd4857e75abbfd9d2580143576aec921566f68bee29580f83397f06eaad7b",
  "699e754edf6629c6c0fdb36f49b29ba4f9972689e7d6eac51c07396e0e79d5a5",
  "1795b11dfd34b39bff2b5e9a3538fad03537e9b0b6f7b437549c144a87e54422",
  "1bd53dab88c66062395337eda76626c48dde0fa66450881a3a273a0779daa3a3",
  "3989d17a5bbeb9c33c16e6145ece569b8ec5530b582a3d572a88ea68166b54e6",
  "755bc8e915739af23e682ccd41bc5f56043587e88ff7cead6e055a0b6f5ebf06",
  "23fb2c533fabe1434ab59c625175c5668313d327b4bd9e85d10b6353aff1a777",
  "1cb6d3ac9a9bd3932066e54eeaa883cfb7eabbf894a897792559b89181a7ec3f",
  "2898ddf79d2ab55c0ec85cb120e38b2ad5b7b3e49ff5b21c0004ba61e51871fa",
  "445107446efbc8320bc2dc01f32a90f0e911f6628d3ebc168a7790f5b631b445",
  "0c1802c18d36479a034d934d964255a133bb7e96a98e9481a6d0df0294bba8a1",
  "6358894ff4bdfb36b03da3474ab6b485f369f3603d108a0427b6da37d10dee9a",
  "15c2681c0db7b4802673b5d23ea2fdd4fa27b5243bbca4d662592737e0343d33",
  "1e9f0950e2eebfff1d1710911a77cdeaff3bc7f08e90219b1bd6b9b6bdb35439",
  "353f364824a2b2bf619c9e99909db021e1eddd722252c0a57b25f4872504503f",
  "2675b3629ded5035e7a7db1c948b3446f5ceb8586845ea016dc281ab29f4452d",
  "1b115d2a761dc29cea966d68621afb099ffe745ac66d62a97035de1b88ba4eb3",
  "309ad7627efacd256cadd75a2f0b3c2d4e64f2f212dd35903ff83ce079d0d658",
  "3a973560eee369826408e3283385a8c2f816cfd23481f44f2e9229ec66b902e8",
  "2f32762e3acc02d0723567325c4c93c553ca4c7f476e8daddfa00e6d72555757",
  "28348fa224859325b1727d3a96d80b33b4ecd6d86f2b0ec49d46a7e4462b2242",
  "7fb4d7f17afd82cd54c856beb63292e1ef7d6755de3ebd4f1244758a31c1dfcb",
  "608c52ca743789fcab03a4bdd15f01caca7202097ea9479e3ec708ff7f353cbe",
  "1679b81eb0430bf7af51fda0f6fd82e8f86669a83dbe52fa85203c09b42c3224",
  "1418bcb5f6d900d465143d2a0746a6f0b26a0e50b42c291c86e36a3d988b988f",
  "406c256941f15d5be758f32253bd642591d3e9f1791a76ed099166b5704bf659",
  "6c2751548afc279175356c4de3b91b35c954621a211092d80ccca6260aa87735",
  "2a1994b055022f49babee488e9ca1175f1a30e417bb3be66db838d7adf06356d",
  "559beae1ad6c70d9e6134fe99b220fdd08506d3ce8f8ac10a9f5ebd18ab2f5e9",
  "44c3d16831b57d8ea4934108b96a0d1905a4db3b74edc6f9c613243ad91ee5e9",
  "7148ba319870b53dffb7910d187fe2003671773be196d5d354545ff8a54b0c3d",
  "13a2f87e5e8d2ca8cc8faa0569504066bd53745b836b03f83b33bf996009fd20",
  "43b97830cfa9ecc74acee4cdbe184702e45f1a8eff8e12c5dc367e596df6b613",
  "39e6be88f529b8ff16f51c8af49afd6cc59dd30450518b545411f69f7da86608",
  "6759cbd2e0ef56b13aa7880dc475102e4f8774fc3cd3b81a2826a95f2b7634a1",
  "1e09d48cea696755955ced3e70bee5a6d60d0083c4bcaa617f0b7584eb0edf13",
  "374bbd36d19132b75b612e27b6598f81695f9d05abe827cef4372ea29597fd75",
  "2f1a69a85e56384ecce7546d5826dd59801750cba06d9c3d757b42eb7426c34d",
  "572791401101738f8d2ff5ad07f7ca3bf5e3c738aef9b9067bb83a826f283da9",
  "056e7f60a69c71340e8c414d1ae55f4750659bf1d731c442971b8a39acc07606",
  "23392b3f4a97dee9716a7ee462c82e08783bd71ba921c0413761377106d7cc7b",
  "0b8802f67d31af02c83bd46eb02158ae044767496d6402628a640fb1a78ae7a3",
  "6d0ff6ff07ba54552e00941df4cca7de36a91eb011a11ef7f1898fbfba70707b",
  "395ed3429464f1746ce2684fb594dfdb3048858c414c8cb156b566fd4cf2a9c3",
  "469c93b65e8042f1238de7758d1702fc28961fbca8a34a4dbcafb451f30cc1fe",
  "1ebcb2ed3f6f50b8a9081309925606d46c8ed470237f4825fbf8d425d7c033f0",
  "33d4e29496b5b2ef1de63239890ad703e134acb380ee004f2550038da9338843",
  "32963426b9f0f722d2b05ae5362728e7a7ed9d745d96096e407dce27fe6754a2",
  "2d9c099d9b4711132527d46716ca541808b473ec56164bfb8db418b7465e70ca",
  "1c7fdd73056e67ac2c851e731f7e71975a8252aedc3adc4dbfd37e53a87aa881",
  "05aabdfeb50990f65eb3f6c69af7e3eb3a2cb5e628b3a9c15598832b2eac6981",
  "3add2f8737e629a632e524302063b9be0da65226a8f4dd255dc9f67fda75d873",
  "4bfd258b0ab53a985f9b09f0a4ff68e8f42ce2f560e3735f5f2dca7dcbc35128",
  "1c906d6adc075f9df0140d800b7a5ae3f8e3de1dcbe8f5b596572c66c2145a9f",
  "300253b539abd8602df9e04d350f69b19753f859bff562e344d3f013102fcdc8",
  "3968ba174d67aa574a6787eccd456780da2720f315f797a7e0c79ec600ab7fb8",
  "4adeb3f073120db3de48d2c3b0f04cb2082a1d607c6d41edfc45deb1344ab6dc",
  "72f41060797dfa6dc129cd6e1b19407aab1fa06c222e0f81d06c1beab4d60f4e",
  "32e5c818d8f3cc9a3f6ce63e7685d7c0e3eae8991edcaf05acc6b56224105da0",
  "1548ef7b9c049420d5e061cf489f2aca0845e710fa02ea20ba1ed219f3dd2ff2",
  "39c5f143f09b4067243fcac22cfee4aad6a7b74661b61437647ea68bd2d3b6e0",
  "3120bdb4b1021c49c57d6a318ec204d09aa3281ac42d2fcb687433b0bb49db31",
  "6729820156a8d68b8ae65ff4d891ce61439fb739e285efc8873e9fb66a2bf9e8",
  "132f6585f9dc9aa9ab29081694d4eb9289f59e38aef9353bd8d96863794bbb33",
  "4ddfbc0401719428a282b672afe3572c33777d643b7f9d135faf506d7336072a",
  "1608a36cefe8c98ff6280f8210473eda39a4027b299d7a9cd5e87cd350ba6038",
  "60551c1341006409639e43b493992df073a31fabc42b3a0c7dfc1d33823d158e",
  "1c0833143d6fa03cc50a42fa746f2843e002c51150d0ebda0b14de60f8cd12fb",
  "431c4743daa1d0c4d01fccbfa9af8c7059522315e639b282666c6b5a128f0fde",
  "0044062528749c78fb69f073f016902a35b0ececb167b0f41758fee7a548dc2e",
  "5fc5334a5a5f5097a511d462659d88ed168b6406ea72df86b4578a0b431db168",
  "32a43a17a74f5a2c79006b5a86a6638817419debfe6db18ffe8cb2abbd46deed",
  "357643dfed0e96411e544549b76cddc4908d612653fb9d0ed1f2c520c037d638",
  "02571a45e671231a25c9f5a04f677cce9f20e68790242e2782fbbf56b82a5e3c",
  "7f27cacfa2a150b4fd46f01abc1e2fb30db57c9db78c4e093e6baca5e884c26c",
  "66f8bdf3bcd10d08043ba406bcd8c4fd40bb911dfbb406c7dd331e81366d65d3",
  "060e5186769e1e5d8cc6cde1a53b508de7f6b5adc94b845e7d26d5ecc136b553",
  "691ef777778e61a20223087aedc056f4a15171b5034ba3d64db649a5a8c3c216",
  "6ea919adf966df74311004c640081d2cd7ee56c0506682fda44bcde9a91cf1fc",
  "0b35f1409975c5e3cc15d93e3efb477a38d5190a0bb3d97100c8e719da7bdc54",
  "7fe3bc81ea4c8a1fc73699c179a53bedc042ee43985ee76095e1e68ed46361b9",
  "0cba2c78c0836e9e2d6599d5300a2a485e009d39d3315b7a0e0278a8956eaaa0",
  "22788b6b5d1392bd955d7ba4c25d81f6e0ea33cbd1ce51d63348dea8b0096f48",
  "1b46f4e87398f564467bdb64e3024272c737b8f03d62a27c36ee0cf819aea6ad"
]
```
- verify the block height with block count `bitcoin-cli -regtest getblockcount`

`
bitcoin-cli -regtest getblockcount
101
`


## 3. Learn structure and metadata of btc blocks
- get best blockhash with `bitcoin-cli -regtest getbestblockhash`
`
bitcoin-cli -regtest getbestblockhash
1b46f4e87398f564467bdb64e3024272c737b8f03d62a27c36ee0cf819aea6ad
`
- get block with `bitcoin-cli -regtest getblock <blockhash>`
`
bitcoin-cli -regtest getblock 1b46f4e87398f564467bdb64e3024272c737b8f03d62a27c36ee0cf819aea6ad
{
  "hash": "1b46f4e87398f564467bdb64e3024272c737b8f03d62a27c36ee0cf819aea6ad",
  "confirmations": 1,
  "height": 101,
  "version": 536870912,
  "versionHex": "20000000",
  "merkleroot": "438f0c04cab2276438333dc1feb21549acd2b45b369188376acdf748aa1f0c0e",
  "time": 1760788793,
  "mediantime": 1760788792,
  "nonce": 2,
  "bits": "207fffff",
  "target": "7fffff0000000000000000000000000000000000000000000000000000000000",
  "difficulty": 4.656542373906925e-10,
  "chainwork": "00000000000000000000000000000000000000000000000000000000000000cc",
  "nTx": 1,
  "previousblockhash": "22788b6b5d1392bd955d7ba4c25d81f6e0ea33cbd1ce51d63348dea8b0096f48",
  "strippedsize": 213,
  "size": 249,
  "weight": 888,
  "tx": [
    "438f0c04cab2276438333dc1feb21549acd2b45b369188376acdf748aa1f0c0e"
  ]
}
`
- get block header with `bitcoin-cli -regtest getblockheader 1b46f4e87398f564467bdb64e3024272c737b8f03d62a27c36ee0cf819aea6ad`
`
bitcoin-cli -regtest getblockheader 1b46f4e87398f564467bdb64e3024272c737b8f03d62a27c36ee0cf819aea6ad
{
  "hash": "1b46f4e87398f564467bdb64e3024272c737b8f03d62a27c36ee0cf819aea6ad",
  "confirmations": 1,
  "height": 101,
  "version": 536870912,
  "versionHex": "20000000",
  "merkleroot": "438f0c04cab2276438333dc1feb21549acd2b45b369188376acdf748aa1f0c0e",
  "time": 1760788793,
  "mediantime": 1760788792,
  "nonce": 2,
  "bits": "207fffff",
  "target": "7fffff0000000000000000000000000000000000000000000000000000000000",
  "difficulty": 4.656542373906925e-10,
  "chainwork": "00000000000000000000000000000000000000000000000000000000000000cc",
  "nTx": 1,
  "previousblockhash": "22788b6b5d1392bd955d7ba4c25d81f6e0ea33cbd1ce51d63348dea8b0096f48"
}
`


## 4. managing multiple wallet and addresses
- create new wallet : `bitcoin-cli -regtest createwallet "testwallet"`
- generate some addresses from it using `bitcoin-cli -regtest -rpcwallet="testwallet" getnewaddress`
**Run this bash script [bash_script](./bitcoin_loop_script.sh)**
`
Starting Bitcoin Loop Script - 3 iterations
=============================================

=== ITERATION 1 ===
bcrt1q8zsznycysk5rvxr0rjcjuu09yft0arpvxzsazg
=== END ITERATION 1 ===


=== ITERATION 2 ===
bcrt1qk70nc0x53a0sk5vzelnwhyur9dqv96mlqxtdrr
=== END ITERATION 2 ===


=== ITERATION 3 ===
bcrt1qyc2zw8e8qn8zmtl9cuwr8642vx0c6nd6ce7lvu
=== END ITERATION 3 ===

Script completed! All 3 iterations finished.
Final blockchain state:
101
"regtest"
1
addresses controlled by my wallet:
[
  {
    "address": "bcrt1qyc2zw8e8qn8zmtl9cuwr8642vx0c6nd6ce7lvu",
    "amount": 0.00000000,
    "confirmations": 0,
    "label": "",
    "txids": [
    ]
  },
  {
    "address": "bcrt1q8zsznycysk5rvxr0rjcjuu09yft0arpvxzsazg",
    "amount": 0.00000000,
    "confirmations": 0,
    "label": "",
    "txids": [
    ]
  },
  {
    "address": "bcrt1qk70nc0x53a0sk5vzelnwhyur9dqv96mlqxtdrr",
    "amount": 0.00000000,
    "confirmations": 0,
    "label": "",
    "txids": [
    ]
  }
]
`


## 5. Send and track transactions
- Firstly mine some btc after 101 confirmations to one of the addresses derived `bitcoin-cli -regtest -rpcwallet="testwallet" generatetoaddress 101 bcrt1qyc2zw8e8qn8zmtl9cuwr8642vx0c6nd6ce7lvu`
- after 404 blocks with blockhash *1f4e8955cea20dd5fd3ae6d4791c452e1066b6040eef990a278d803e500ad235*, we have 6212.5 regtest btc as of writing
- Load the "mywallet" : `bitcoin-cli -regtest loadwallet "mywallet"`
- Generate an address with the "mywallet" : `bitcoin-cli --regtest -rpcwallet="mywallet" getnewaddress` : bcrt1qr554cne0ppvukc333j4kxvmfv5ryfjdhfpyl06
- sending 3 btc to one of the other wallets we generated i.e "mywallet" after setting fallback fee to 0.00001 in the bitcoin.conf file `bitcoin-cli -regtest -rpcwallet="testwallet" sendtoaddress bcrt1qr554cne0ppvukc333j4kxvmfv5ryfjdhfpyl06 3.0 "" "" false true null "unset" false 1` with txid : **d85dbf1b63d21a4e6e1d8c487ca45d68c02706a4ebfc68bbae7314b58750a020**
- Balance of sender wallet is 6334.49999859
- Balance of reciever wallet is 5053.00000000


## 6. Inspect UTXO
- use `bitcoin-cli -regtest -rpcwallet=<walletname> listunspent` to get unspent utxo
There were a lot I will just drop one instance of a UTXO
`
{
    "txid": "330e61d9727563c3e69d47c3d52eeb33dda98a6cd96148f60f27dfb71d9ba01e",
    "vout": 0,
    "address": "bcrt1qyc2zw8e8qn8zmtl9cuwr8642vx0c6nd6ce7lvu",
    "label": "",
    "scriptPubKey": "00142614271f2704ce2dafe5c71c33eaaa619f8d4dba",
    "amount": 25.00000000,
    "confirmations": 127,
    "spendable": true,
    "solvable": true,
    "desc": "wpkh([026a84f2/84h/1h/0h/0/2]032b8130089ce5b3cb624a0ed7161d497e174951201f9295d5ea5d9104a83bc18a)#muwq0xjj",
    "parent_descs": [
      "wpkh([026a84f2/84h/1h/0h]tpubDChQ7xKNLExxgByVyJxrs3vtYxPWSXHqQTfifEpEH4vGuxkfjjdpdZYschXqmLTGnCgKTobyDNweXJSEuSEuHv9dHfaZAsERupVKYSFhGDa/0/*)#ju6yv9y3"
    ],
    "safe": true
  },
`


## 7. Decode a raw transaction
- First get the transcation using the gettransaction `bitcoin-cli -regtest -rpcwallet="testwallet" gettransaction d85dbf1b63d21a4e6e1d8c487ca45d68c02706a4ebfc68bbae7314b58750a020`
- get raw tx using `bitcoin-cli -regtest getrawtransaction <tx_id> true <blockhash>`
The notable parameters are :
1. in_active_chain
2. txid
3. hash
4. version
5. size and vsize
6. weight
7. vin
8. vout
9. hex
10. confirmation, time, blocktime


## 8. Simulate a tx flow
pls check 5