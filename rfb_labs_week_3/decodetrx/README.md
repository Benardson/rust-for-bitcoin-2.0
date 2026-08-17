# Bitcoin Transaction Serializer

A Rust CLI application for constructing and serializing Bitcoin transactions.

## Features
- Transaction version
- Multiple inputs
- Multiple outputs
- SegWit witness data
- Multiple witness items
- Locktime
- Hexadecimal validation
- Bitcoin CompactSize serialization
- Serialized transaction hexadecimal output
- Transaction size in bytes

## Requirements
- Rust
- Cargo

## Usage
cargo run -- --help

### Input format
TXID:VOUT:SEQUENCE:SCRIPTSIG
Repeat --input for multiple inputs.

### Output format
AMOUNT_IN_SATOSHIS:SCRIPTPUBKEY
Repeat --output for multiple outputs.

### Witness format
INPUT_INDEX:ITEM_HEX
Repeat --witness for multiple witness items.

## Serialization
Bitcoin transaction integers use little-endian encoding and variable-length fields use CompactSize encoding.
SegWit transactions include the marker, flag, and witness data.

## Validation
User-provided hexadecimal values are validated before serialization. Invalid values produce meaningful errors instead of panics.

## Output
The program displays the transaction version, SegWit status, input/output counts, witness status, locktime, serialized transaction hexadecimal, and transaction size in bytes.
