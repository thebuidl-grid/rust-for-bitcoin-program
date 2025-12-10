#!/bin/bash

echo "==================================="
echo "Simple Script Execution Examples"
echo "==================================="
echo ""

echo "Example 1: Push a value and check if true"
echo "Script: 0x01 0x42 (PUSH 1 byte: 0x42)"
cargo run -- execute-script "0142" --verbose
echo ""

echo "==================================="
echo "Example 2: Push and duplicate"
echo "Script: 0x01 0x42 0x76 (PUSH 0x42, OP_DUP)"
cargo run -- execute-script "010142" --verbose
echo ""

echo "==================================="
echo "Example 3: Hash160 operation"
echo "Script: Push 'hello' and hash it"
SCRIPT="0568656c6c6fa9"
echo "Script hex: $SCRIPT"
cargo run -- execute-script "$SCRIPT" --verbose
echo ""

echo "==================================="
echo "Example 4: Test equality"
echo "Script: Push two same values and verify equal"
SCRIPT="0142014287"
echo "Script hex: $SCRIPT (PUSH 0x42, PUSH 0x42, OP_EQUAL)"
cargo run -- execute-script "$SCRIPT" --verbose
echo ""
