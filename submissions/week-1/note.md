### 1. Verify Your Node Setup: Ensure your node is correctly configured and responsive via CLI

![Screenshot](https://greasy-yellow-7wefaen07t.edgeone.app/Screenshot%20From%202025-10-20%2010-45-23.png)

- chain: regtest
- blocks: 105

### 2. Generate Blocks: Understand block creation and coinbase maturity on regtest

![Screenshot](https://greasy-yellow-7wefaen07t.edgeone.app/Screenshot%20From%202025-10-20%2011-05-42.png)

- block count: 206

### 3. Explore the Blockchain: Learn the structure and metadata of Bitcoin blocks

![screenshot](https://married-amethyst-vc2nj8y2j4.edgeone.app/Screenshot%20From%202025-10-20%2018-47-05.png) (https://stiff-turquoise-thgm1l4w7o.edgeone.app/Screenshot%20From%202025-10-20%2018-47-57.png)

### 4. Work with Wallets: Get comfortable managing multiple wallets and addresses

![screenshot](http://greasy-yellow-7wefaen07t.edgeone.app/Screenshot%20From%202025-10-20%2010-58-23.png)\n(https://greasy-yellow-7wefaen07t.edgeone.app/Screenshot%20From%202025-10-20%2011-01-03.png) \n ()


- "testwallet" already exist, so I created few new addresses

### 5. Send and Track Transactions: Learn how transactions are created, broadcast, and confirmed

![screenshot](https://dominant-cyan-mggz9znqla.edgeone.app/Screenshot%20From%202025-10-20%2018-56-01.png)

- I had to specify the fee_rate for the txn to go

### 6. Inspect UTXOs: Send and Track Transactions: Understand how UTXOs represent spendable Bitcoin

![screenshot](https://dominant-cyan-mggz9znqla.edgeone.app/Screenshot%20From%202025-10-20%2019-00-58.png)

- txid: 631515746bdb98691d8897d38a6f55572a4ba3ffe03c40628d30a610be3e94ef
- vout: 0
- amount: 50.00000000


### 8. Simulate a Payment Workflow: Experience the full transaction lifecycle in regtest

#### Commands Used

- `bitcoin-cli createwallet "sender"`

```zsh
bitcoin-cli createwallet "senders"
{
  "name": "senders"
}
```

- `bitcoin-cli createwallet "receivers"`

```zsh
bitcoin-cli createwallet "receivers"
{
  "name": "receivers"
}
```

- `bitcoin-cli -rpcwallet=senders getnewaddress`

```zsh
bitcoin-cli -rpcwallet=senders getnewaddress
bcrt1qvwcsq7fm9p3k8u0dzndv6gm4upnmpcfysdx9vs
```

- `bitcoin-cli -rpcwallet=senders generatetoaddress 101 bcrt1qvwcsq7fm9p3k8u0dzndv6gm4upnmpcfysdx9vs`

```zsh
bitcoin-cli -rpcwallet=sender generatetoaddress 101 bcrt1qvwcsq7fm9p3k8u0dzndv6gm4upnmpcfysdx9vs
[
  "186515002fb8c0d12a23f390e21e86ca2605e74ad1b1e092cf38aeaca9fb7018",
  "10e167b63d3f241d014f6fa4f825fa293583e4c5198266f5787aa2236be415dc",
  "7ef37635bf88e29973bb7ba513f87e6e694473a69818b38d6d1f3414be5a8d92",
  "6101ed66396402d72f2773f8db60c1877d6682175241efbad5794c557c0ca247",
  "2d76fe2de9fff290c7e2d084135fdd0130a8abd823f12de87ddd1283c9e71dc7",
  "6a3b762454877f2c0afb01ec4e74a60506c8ab4c61262349d0ef5202c037d6ae",
  "6dbe62871beb3ea65e68d3f3f7356cf419e736b7036248c395689eeecd2d31eb",
  "7752e4047184557481205c182a193dc1e6e3c6068407e1a9f20f02bf8fb01501",
  "43f1df75a263e6fc3b5845ca1188eaa092511157edf49cb2c1e237c4d485b842",
  "4816de567851057a5f066e4d84ce16faf13ac9f45960cd3bb237248881f0d529",
  "7ba2f5909ec7597a3692218ba4951df3e3396f19609c766ff286ee386f9dc843",
  "70425f4cb3f959468732816b52fdc710f65cc077e65c0116a88e5cc9662ff6ed",
  "0af00ae7e624b6307a5941970d3fad57c558d3d8f650272b9cc238f26b019433",
  "308ca7ea650dd3da67f7116b7ae55c5ded24ef6b8c3eab564f7d9ceddb83e204",
  "4817a57bd7a547b5bf3cb437b0531509ca74913a738bcd06f121f54cb4865583",
  "7900044f79aa5b1f541f4fa808b4628a0b74c8f618a3d904b8c9d466e138e820",
  "4689e610beec5f2e539e6879ea207a5608746e00136a677234abf82d9abc9675",
  "3cf9d0234ec11a733790862cbcdc9b7ca57fff058df0e45b3d83bc5d0fade306",
  "1d01269fc5f1fbc300b3374c9cb5617dc8ce6742ac40920e526f3c23ebf86d3e",
  "1227f36251cc5905b3237c3c865016620cab71e5d4897e45e2edfea88dd88a5d",
  "482f7240bdc1e103421f6e8bb53585bd5e4ecb4254a427cd969303c86e186440",
  "3ada6e26983af938dd0e9f6c54f605635947f05a48f70dd58a80ed3e3fed9233",
  "54b67954c9e52a056372f2af3f921efb3aec7075d1e116bb815d8dc09624569e",
  "4795d27e8a7178da178b673afa32e5334a8e5dd3e902269e7bcdb50588ddcd11",
  "3ba2e45da7ef9c5fe9591a4b8f88138dc8651ff91d363e046b96f70be6ec27d8",
  "6c695a37d82a4559e0032d5109631bc5d543dd76232a280d8a834b1981df66fc",
  "1684e783df0a7001f62f38252b7f3b6321be4507a339f070c1995ee38dd4941a",
  "6f4b2e7b40eeb326d971cd49ed11927673415e4819cdb47a98c5f993646f0ddb",
  "03bfd9c24107c61ecf3faa5c2d0648f4e981851915e73bf9f945858c309b2f98",
  "4cf9d27f043d6b669b9eded2bc7fda9eb0111af8f8e9d333204ffd39b9ebaf3b",
  "6fab43b5a5ec6f37f8230c509c9be0c1b520f4cb9e8d5580678664b367960901",
  "77421166f796d2d810f7ca15b1cea10f4a349960d6f3596e61ac6de270734fa6",
  "1c0545117f451ce34c6f291324c1171925e8d6d233b2c6e01ec105a814f7900e",
  "0a780c01c6cdda69d09ff7c485cc0cbea7485d15450214f68ae035936be9cb76",
  "50917fd9262a6d73fae3a68445755de06a59c6a78af75d57626e98df11640a2e",
  "54142587e9b0f19498d9faa9b40ae559729ae76ef0614e0589f58aa56bdde336",
  "351ed18cad0c61945b4371f9246e74da40834ff442a639476b4aedbae6bf0ef8",
  "45cbc3c60c0ac261fe8ea9605c399dfe39766eafd5ca1129fd4a0bb8f225b77b",
  "46848d04db3e6e5e6f4e689634f609187883434200f0b3955a8c951aad91cc0d",
  "6ba05f4f9807dad5e7fc5c9bb1b00e0638831bfe927d30bc76faeab44b7ae9c7",
  "50547ecb3483a82c5e53196964dbda89601f52a86d425cb52f38b18d77982b0e",

  ...
]
```

- `bitcoin-cli -rpcwallet=sender getbalance`

```zsh
bitcoin-cli -rpcwallet=sender getbalance
3.00000141
```

**I defined a low fallbackfee that's why it's 25.00000141**

- `bitcoin-cli -rpcwallet=receivers getnewaddress`
  
```zsh
bitcoin-cli -rpcwallet=receivers getnewaddress
crt1qscjd77eu0h38r36m29uanvutxlufhsrcshsx9v
```

- `bitcoin-cli -rpcwallet=senders sendtoaddress crt1qscjd77eu0h38r36m29uanvutxlufhsrcshsx9v 3 "" "" false false null "unset" null 1`

```zsh
bitcoin-cli -rpcwallet=senders sendtoaddress crt1qscjd77eu0h38r36m29uanvutxlufhsrcshsx9v 3 
cfd7bcd6385484126a3c4816f180cf0e1b79076c3cec95bb60e89def21b5b01f
```



- `bitcoin-cli -rpcwallet=receiver getbalance`

```zsh
bitcoin-cli -rpcwallet=receiver getbalance
3.00000000
```


