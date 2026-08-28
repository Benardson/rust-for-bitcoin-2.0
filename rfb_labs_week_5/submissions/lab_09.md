# Lab 09 — BIP44 path decoding

## Commands used

- `cargo test --test lab_09 -- --nocapture`
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Terminal output

The required path `m/44'/0'/2'/1/5` decodes as purpose `44`, coin type `0`, account `2`, change branch `1`, and address index `5`. The Lab 09 public suite passed all 4 tests, including path decoding, description, final-index replacement, and P2PKH derivation.

## Evidence references

Terminal evidence is represented by the successful `cargo test --test lab_09` run. The derivation uses the published public mnemonic and disposable test output.

## Explanation

The first hardened level, `44'`, identifies the BIP44 purpose. `0'` is the Bitcoin coin type in the standard BIP44 convention. `2'` selects the third account because account numbering is zero-based. The `1` branch represents change, while `0` would represent external receiving addresses. The final `5` is the zero-based address index. Apostrophes mark hardened derivation levels.
