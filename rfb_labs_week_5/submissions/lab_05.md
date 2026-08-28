# Lab 05 — Address compatibility map

## Commands used

- `cargo test --test lab_05 -- --nocapture`
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Terminal output

The Lab 05 public suite passed all 4 tests. The implementation maps Base58Check P2PKH, Base58Check P2SH, Bech32, and Bech32m capabilities to P2PKH, wrapped SegWit, native SegWit, and Taproot receiving formats.

## Evidence references

Terminal evidence is represented by the successful `cargo test --test lab_05` run. No real wallet credentials were used.

## Explanation

An older wallet may understand Base58Check P2SH addresses beginning with `3` while not understanding Bech32 addresses beginning with `bc1q`. The ability to send to an address depends on whether the sender can decode and construct the corresponding output. Spending is a different capability because it requires the appropriate private key and signing/witness logic. A wallet can therefore support receiving or sending to some address formats without supporting every spending mechanism.
