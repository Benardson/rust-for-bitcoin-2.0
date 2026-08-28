use bip39::{Language, Mnemonic};
use bitcoin::bip32::{ChildNumber, DerivationPath, Xpriv, Xpub};
use bitcoin::{Network, PrivateKey};

use crate::error::LabError;
use crate::model::ExtendedKeyReport;
use crate::LabResult;

/// Create the master extended private key from a BIP39 recovery setup.
pub fn master_xpriv(mnemonic: &str, passphrase: &str, network: Network) -> LabResult<String> {
    let normalized = mnemonic.split_whitespace().collect::<Vec<_>>().join(" ");

    let mnemonic = Mnemonic::parse_in_normalized(Language::English, &normalized)
        .map_err(|e| LabError::InvalidMnemonic(e.to_string()))?;

    let seed = mnemonic.to_seed(passphrase);

    let xpriv =
        Xpriv::new_master(network, &seed).map_err(|e| LabError::Derivation(e.to_string()))?;

    Ok(xpriv.to_string())
}

/// Derive an extended private/public key pair at a complete path.
pub fn derive_extended_keys(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    network: Network,
) -> LabResult<ExtendedKeyReport> {
    let normalized = mnemonic.split_whitespace().collect::<Vec<_>>().join(" ");

    let mnemonic = Mnemonic::parse_in_normalized(Language::English, &normalized)
        .map_err(|e| LabError::InvalidMnemonic(e.to_string()))?;

    let seed = mnemonic.to_seed(passphrase);

    let master =
        Xpriv::new_master(network, &seed).map_err(|e| LabError::Derivation(e.to_string()))?;

    let derivation_path = path
        .parse::<DerivationPath>()
        .map_err(|e| LabError::InvalidPath(e.to_string()))?;

    let secp = bitcoin::secp256k1::Secp256k1::new();

    let derived = master
        .derive_priv(&secp, &derivation_path)
        .map_err(|e| LabError::Derivation(e.to_string()))?;

    let xpub = Xpub::from_priv(&secp, &derived);

    Ok(ExtendedKeyReport {
        derivation_path: path.to_owned(),
        xpriv: derived.to_string(),
        xpub: xpub.to_string(),
    })
}

/// Derive a normal public child from an xpub without private key material.
pub fn derive_normal_child_xpub(parent_xpub: &str, index: u32) -> LabResult<String> {
    if index >= 1 << 31 {
        return Err(LabError::InvalidPath(
            "public child index must be non-hardened".to_owned(),
        ));
    }

    let xpub = parent_xpub
        .parse::<Xpub>()
        .map_err(|e| LabError::InvalidKey(e.to_string()))?;

    let child_number = ChildNumber::Normal { index };

    let secp = bitcoin::secp256k1::Secp256k1::new();

    let child = xpub
        .derive_pub(&secp, &child_number)
        .map_err(|e| LabError::Derivation(e.to_string()))?;

    Ok(child.to_string())
}

/// Return whether a textual path contains at least one hardened step.
pub fn path_contains_hardened_step(path: &str) -> LabResult<bool> {
    let derivation_path = path
        .parse::<DerivationPath>()
        .map_err(|e| LabError::InvalidPath(e.to_string()))?;

    Ok(derivation_path
        .into_iter()
        .any(|child| matches!(child, ChildNumber::Hardened { .. })))
}
