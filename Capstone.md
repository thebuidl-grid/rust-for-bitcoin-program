
Bitcoin Projects

Project 1: Bitcoin Script Interpreter
Build a program to execute Bitcoin scripts and validate transactions. Implement an engine that processes opcodes (`OP_DUP`, `OP_HASH160`, `OP_EQUALVERIFY`, `OP_CHECKSIG`) using a stack, with detailed execution logging. Test for valid and invalid Pay-to-Public-Key-Hash (P2PKH) scripts.

Project 2: Block Explorer Backend
Create a REST API backend to parse Bitcoin transaction files, index blocks and transactions, and serve data from a database. Provide endpoints such as:
- `/block/{hash}`
- `/tx/{txid}`

Project 3: Mini Blockchain Clone
Build a simplified Bitcoin blockchain with:
- Proof-of-work mining
- Longest chain selection
- Disk persistence
Use a CLI for mining, viewing, and validating blocks.

Project 4: Hashing & Key Toolkit
Develop a CLI tool and Rust library for Bitcoin-related cryptography:
- SHA256
- RIPEMD160
- HASH160
- secp256k1 key generation
Show address derivation and offer simple CLI commands for each function.


Project 5: Elliptic Curve Math Engine
Implement 256-bit modular arithmetic for secp256k1:
- Addition
- Subtraction
- Multiplication
- Inversion
Use Rust operator overloading and unit tests. Show applications in ECDSA, point multiplication, and public key derivation.


Project 6: Build an application with any use case that leverages lightening node for bitcoin payments.

Project 7: Build an application with any use case that leverages lightening node for bitcoin payments.
