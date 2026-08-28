use bip39::{Language, Mnemonic};

use crate::model::{MnemonicReport, PassphraseComparison};
use crate::LabResult;

/// Validate an English mnemonic and report its entropy/checksum structure.
pub fn inspect_mnemonic(mnemonic: &str) -> LabResult<MnemonicReport> {
    let normalized = mnemonic.split_whitespace().collect::<Vec<_>>().join(" ");

    let mnemonic = Mnemonic::parse_in_normalized(Language::English, &normalized)
        .map_err(|e| crate::error::LabError::InvalidMnemonic(e.to_string()))?;

    let word_count = mnemonic.word_count();

    let entropy_bits = match word_count {
        12 => 128,
        15 => 160,
        18 => 192,
        21 => 224,
        24 => 256,
        _ => {
            return Err(crate::error::LabError::InvalidMnemonic(
                "invalid BIP39 word count".to_owned(),
            ))
        }
    };

    let checksum_bits = entropy_bits / 32;

    Ok(MnemonicReport {
        word_count,
        entropy_bits,
        checksum_bits,
    })
}

/// Derive the 512-bit BIP39 seed from words plus an optional passphrase.
pub fn mnemonic_seed_hex(mnemonic: &str, passphrase: &str) -> LabResult<String> {
    let normalized = mnemonic.split_whitespace().collect::<Vec<_>>().join(" ");

    let mnemonic = Mnemonic::parse_in_normalized(Language::English, &normalized)
        .map_err(|e| crate::error::LabError::InvalidMnemonic(e.to_string()))?;

    let seed = mnemonic.to_seed(passphrase);

    Ok(seed
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>())
}

/// Demonstrate that the same words with a different passphrase make a different seed.
pub fn compare_passphrases(
    mnemonic: &str,
    protected_passphrase: &str,
) -> LabResult<PassphraseComparison> {
    let empty_passphrase_seed_hex = mnemonic_seed_hex(mnemonic, "")?;
    let protected_seed_hex = mnemonic_seed_hex(mnemonic, protected_passphrase)?;

    Ok(PassphraseComparison {
        seeds_differ: empty_passphrase_seed_hex != protected_seed_hex,
        empty_passphrase_seed_hex,
        protected_seed_hex,
    })
}

/// Recognize the public BIP39 test mnemonic used in the class labs.
pub fn is_public_test_mnemonic(mnemonic: &str) -> bool {
    let normalized = mnemonic.split_whitespace().collect::<Vec<_>>().join(" ");

    normalized
        == "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
}
