# Lab 02 — Legacy P2PKH

## Commands used

- `cargo test --test lab_02 -- --nocapture`
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Terminal output

The Lab 02 public suite passed all 4 tests. The implementation derives a P2PKH address, constructs the standard P2PKH locking script, exposes the public-key HASH160 commitment, and models signature/public-key data in ScriptSig.

## Evidence references

Terminal evidence is represented by the successful `cargo test --test lab_02` run. Testing used the published/disposable class material only.

## Explanation

P2PKH locks coins to the HASH160 of a public key. The locking script checks that the supplied public key hashes to the committed value and then verifies the signature. The public key identifies the key being used, while the valid signature proves authorization to spend. ScriptSig supplies the signature and public key needed to satisfy those checks.
