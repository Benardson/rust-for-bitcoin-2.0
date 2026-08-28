# Lab 04 — Native P2WPKH

## Commands used

- `cargo test --test lab_04 -- --nocapture`
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Terminal output

The Lab 04 public suite passed all 4 tests. The implementation derives a native regtest `bcrt1q...` address, reports a version-0 20-byte witness program, leaves ScriptSig empty, and places signature/public-key data in the witness.

## Evidence references

Terminal evidence is represented by the successful `cargo test --test lab_04` run. Disposable/public test material was used.

## Explanation

Native P2WPKH places the witness program directly in the output and does not use ScriptSig to carry the spending data. Therefore ScriptSig is empty for a normal native P2WPKH spend, while the signature and public key are supplied in the witness. This differs from legacy P2PKH, which uses ScriptSig, and from P2SH-wrapped SegWit, which has an outer P2SH commitment.
