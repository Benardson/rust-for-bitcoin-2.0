# Lab 08 — BIP32 extended keys

## Commands used

- `cargo test --test lab_08 -- --nocapture`
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Terminal output

The Lab 08 public suite passed all 4 tests. The implementation creates a master xpriv, derives an xpriv/xpub pair at a complete path, derives a normal public child from an xpub, and detects hardened path components. Extended private/public key material is intentionally not reproduced here.

## Evidence references

Terminal evidence is represented by the successful `cargo test --test lab_08` run. Only disposable test data was used and no xpriv was included in the submission.

## Explanation

An xpriv contains private key material together with metadata such as depth, parent fingerprint, child number, and chain code. An xpub exposes the public key and chain code and can support watch-only derivation for normal children. The chain code provides additional key-derivation entropy/state. Hardened derivation uses private parent material and therefore cannot be performed from an xpub alone. Normal non-hardened child derivation can be performed from an xpub.
