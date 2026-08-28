# Lab 10 — Deterministic recovery across address families

## Commands used

- `cargo test --test lab_10 -- --nocapture`
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Terminal output

The Lab 10 public suite passed all 4 tests. The implementation derives BIP44 P2PKH, BIP49 P2SH-P2WPKH, and BIP84 P2WPKH addresses on regtest, confirms repeatability from identical recovery inputs, and confirms that changing only the final index changes the derived address.

## Evidence references

Terminal evidence is represented by the successful `cargo test --test lab_10` run. Only the published test mnemonic and disposable test derivation data were used.

## Explanation

Deterministic recovery works because the mnemonic, passphrase, network, derivation path, and script/address convention are deterministic inputs. Repeating those same inputs reproduces the same private/public key hierarchy and therefore the same address. Changing only the final index selects a different child key and normally produces a different address. Restoring a wallet therefore requires not only the recovery phrase and passphrase but also the correct derivation paths and address/script conventions.
