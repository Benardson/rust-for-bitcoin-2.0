# Lab 07 — BIP39 mnemonic and seed

## Commands used

- `cargo test --test lab_07 -- --nocapture`
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Terminal output

The Lab 07 public suite passed all 4 tests. The published 12-word English test mnemonic is recognized as 128 bits of entropy plus a 4-bit checksum. The implementation derives the 512-bit BIP39 seed and confirms that changing the optional passphrase changes the resulting seed.

## Evidence references

Terminal evidence is represented by the successful `cargo test --test lab_07` run. Only the published class test mnemonic was used; no real recovery phrase was submitted.

## Explanation

Entropy is the original random input. BIP39 adds a checksum derived from that entropy and converts the combined bits into mnemonic words. The mnemonic represents the entropy plus checksum, while the seed is derived from the mnemonic and optional passphrase using the BIP39 key-stretching process. The checksum detects certain errors but is not encryption. A forgotten passphrase cannot be recovered from the mnemonic alone because it changes the derived seed.
