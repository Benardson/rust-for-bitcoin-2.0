# Lab 03 — P2SH 2-of-3 multisig

## Commands used

- `cargo test --test lab_03 -- --nocapture`
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Terminal output

The Lab 03 public suite passed all 4 tests. The implementation builds the `2 <pub1> <pub2> <pub3> 3 OP_CHECKMULTISIG` redeemScript, derives the P2SH address, constructs the outer scriptPubKey, and reports both validation layers.

## Evidence references

Terminal evidence is represented by the successful `cargo test --test lab_03` run. Only disposable class test keys were used.

## Explanation

P2SH first commits to the HASH160 of the redeemScript. When spending, the supplied redeemScript must hash to the committed value. After that outer check succeeds, the redeemScript itself is executed. In this case it requires two valid signatures from the three public keys. Matching the script hash alone therefore does not authorize the spend.
