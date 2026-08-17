# Bitcoin Transaction Serializer

A Rust CLI application for constructing and serializing Bitcoin transactions without modifying the Rust source code.

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
- Meaningful validation errors

## Requirements

- Rust
- Cargo

## Usage

````powershell
cargo run -- --help
``` 

### Input format

````text
TXID:VOUT:SEQUENCE:SCRIPTSIG
``` 

Repeat --input for multiple inputs.

### Output format

````text
AMOUNT_IN_SATOSHIS:SCRIPTPUBKEY
``` 

Repeat --output for multiple outputs.

### Witness format

````text
INPUT_INDEX:ITEM_HEX
``` 

Repeat --witness for multiple witness items.

## Serialization

Bitcoin transaction integers use little-endian encoding and variable-length fields use CompactSize encoding.

SegWit transactions include the marker, flag, and witness data.

## Validation

User-provided hexadecimal values are validated before conversion into bytes. Invalid values produce meaningful errors instead of panics.

## Output

The program displays transaction version, SegWit status, input/output counts, witness data, locktime, serialized transaction hexadecimal, and transaction size in bytes.

## Verification

Run cargo fmt -- --check, cargo check, and cargo build before submission.

