# Lab 06 — Weight, virtual size, and fees

## Commands used

- `cargo test --test lab_06 -- --nocapture`
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Terminal output

The Lab 06 public suite passed all 4 tests. The implementation calculates BIP141 transaction weight, rounds weight upward to virtual bytes, calculates fees from sat/vB, and reproduces the class comparison of approximately 226 vB for P2PKH versus 141 vB for P2WPKH at 50 sat/vB.

## Evidence references

Terminal evidence is represented by the successful `cargo test --test lab_06` run. The fee comparison uses the disposable class example rather than real transaction funds.

## Explanation

BIP141 assigns different weight to transaction data depending on whether it is base data or witness data. Transaction weight is calculated as `3 × stripped_size + total_size`, and virtual size is the weight divided by four with rounding upward. Witness data therefore contributes less weight than equivalent non-witness data, but it is not simply deleted or treated as one flat discount applied to the whole transaction.
