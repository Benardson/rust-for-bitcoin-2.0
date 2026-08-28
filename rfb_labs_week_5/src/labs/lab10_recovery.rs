//! Lab 10 — prove deterministic recovery across BIP44, BIP49, and BIP84.

use bip39::{Language, Mnemonic};
use bitcoin::bip32::{DerivationPath, Xpriv};
use bitcoin::{Address, CompressedPublicKey, Network, PublicKey};

use crate::model::{AddressFormat, DerivedAddressSet};
use crate::LabResult;

/// Derive one address from an arbitrary full path and selected script family.
pub fn derive_address_for_path(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    format: AddressFormat,
    network: Network,
) -> LabResult<String> {
    // Parse the BIP39 mnemonic.
    let mnemonic = Mnemonic::parse_in(Language::English, mnemonic)
        .map_err(|e| crate::LabError::Derivation(e.to_string()))?;

    // Convert the mnemonic and passphrase into the BIP39 seed.
    let seed = mnemonic.to_seed(passphrase);

    // Create the BIP32 master extended private key.
    let master = Xpriv::new_master(network, &seed)
        .map_err(|e| crate::LabError::Derivation(e.to_string()))?;

    // Parse the complete BIP32 derivation path.
    let derivation_path: DerivationPath = path
        .parse::<DerivationPath>()
        .map_err(|e| crate::LabError::InvalidPath(e.to_string()))?;

    // Derive the requested child key.
    let secp = bitcoin::secp256k1::Secp256k1::new();

    let derived = master
        .derive_priv(&secp, &derivation_path)
        .map_err(|e| crate::LabError::Derivation(e.to_string()))?;

    // Get the underlying secp256k1 public key.
    let secp_public_key = derived.private_key.public_key(&secp);

    // Bitcoin PublicKey is used for P2PKH.
    let public_key = PublicKey::new(secp_public_key);

    // CompressedPublicKey is used for P2WPKH and P2SH-P2WPKH.
    let compressed = CompressedPublicKey(secp_public_key);

    let address = match format {
        // BIP44-style legacy P2PKH.
        AddressFormat::P2pkh => Address::p2pkh(public_key, network),

        // BIP84-style native SegWit P2WPKH.
        AddressFormat::P2wpkh => Address::p2wpkh(&compressed, network),

        // BIP49-style wrapped SegWit P2SH-P2WPKH.
        AddressFormat::P2sh => Address::p2shwpkh(&compressed, network),

        // Lab 10 only covers the three address families above.
        _ => {
            return Err(crate::LabError::Derivation(
                "unsupported address format for Lab 10".to_string(),
            ))
        }
    };

    Ok(address.to_string())
}

/// Derive index `n` on the BIP44, BIP49, and BIP84 receive branches.
pub fn derive_address_set(
    mnemonic: &str,
    passphrase: &str,
    account: u32,
    index: u32,
    network: Network,
) -> LabResult<DerivedAddressSet> {
    // BIP44: m/44'/coin_type'/account'/change/index
    let bip44_path = format!("m/44'/1'/{}'/0/{}", account, index);

    // BIP49: m/49'/coin_type'/account'/change/index
    let bip49_path = format!("m/49'/1'/{}'/0/{}", account, index);

    // BIP84: m/84'/coin_type'/account'/change/index
    let bip84_path = format!("m/84'/1'/{}'/0/{}", account, index);

    let bip44_p2pkh = derive_address_for_path(
        mnemonic,
        passphrase,
        &bip44_path,
        AddressFormat::P2pkh,
        network,
    )?;

    let bip49_p2sh_p2wpkh = derive_address_for_path(
        mnemonic,
        passphrase,
        &bip49_path,
        AddressFormat::P2sh,
        network,
    )?;

    let bip84_p2wpkh = derive_address_for_path(
        mnemonic,
        passphrase,
        &bip84_path,
        AddressFormat::P2wpkh,
        network,
    )?;

    Ok(DerivedAddressSet {
        bip44_p2pkh,
        bip49_p2sh_p2wpkh,
        bip84_p2wpkh,
    })
}

/// Prove that identical mnemonic, passphrase, path, and network reproduce an address.
pub fn recovery_is_repeatable(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    format: AddressFormat,
    network: Network,
) -> LabResult<bool> {
    let first = derive_address_for_path(mnemonic, passphrase, path, format, network)?;

    let second = derive_address_for_path(mnemonic, passphrase, path, format, network)?;

    Ok(first == second)
}

/// Prove that changing only the final index selects a different address.
pub fn changing_index_changes_address(
    mnemonic: &str,
    passphrase: &str,
    first_path: &str,
    second_path: &str,
    format: AddressFormat,
    network: Network,
) -> LabResult<bool> {
    let first = derive_address_for_path(mnemonic, passphrase, first_path, format, network)?;

    let second = derive_address_for_path(mnemonic, passphrase, second_path, format, network)?;

    Ok(first != second)
}
