# Lab 01 — Address and network identification

## Commands used

- `cargo test --test lab_01 -- --nocapture`
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Terminal output

The Lab 01 public suite passed all 4 tests. The implementation identifies P2PKH, P2SH, P2WPKH, and P2TR formats, validates addresses against the requested network, and returns the encoded scriptPubKey. Regtest prefixes checked include `m/n`, `2`, `bcrt1q`, and `bcrt1p`.

## Evidence references

Terminal evidence is represented by the successful `cargo test --test lab_01` run. No real wallet addresses or private material were used.

## Explanation

A textual prefix is useful for recognizing the likely address family, but it is not complete validation. Base58Check addresses require checksum validation, and Bech32/Bech32m addresses require their checksum and encoding rules to be validated. The intended Bitcoin network must also match. The implementation therefore parses the address with rust-bitcoin and explicitly requires the requested network before reporting its script family and scriptPubKey.
