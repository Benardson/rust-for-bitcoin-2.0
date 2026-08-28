use bitcoin::bip32::{DerivationPath, ExtendedPrivKey};
use bitcoin::{Address, Network, PublicKey};

use crate::model::Bip44PathInfo;
use crate::LabResult;

pub fn decode_bip44_path(path: &str) -> LabResult<Bip44PathInfo> {
    let parts: Vec<&str> = path.split('/').collect();

    if parts.len() != 6 || parts[0] != "m" {
        return Err(crate::LabError::InvalidPath(
            "BIP44 path must be m/purpose'/coin'/account'/change/index".to_string(),
        ));
    }

    fn hardened(value: &str, name: &str) -> LabResult<u32> {
        let number = value
            .strip_suffix('\'')
            .ok_or_else(|| crate::LabError::InvalidPath(format!("{name} must be hardened")))?;

        number
            .parse::<u32>()
            .map_err(|_| crate::LabError::InvalidPath(format!("invalid {name}: {value}")))
    }

    fn normal(value: &str, name: &str) -> LabResult<u32> {
        if value.ends_with('\'') {
            return Err(crate::LabError::InvalidPath(format!(
                "{name} must not be hardened"
            )));
        }

        value
            .parse::<u32>()
            .map_err(|_| crate::LabError::InvalidPath(format!("invalid {name}: {value}")))
    }

    Ok(Bip44PathInfo {
        purpose: hardened(parts[1], "purpose")?,
        coin_type: hardened(parts[2], "coin type")?,
        account: hardened(parts[3], "account")?,
        change: normal(parts[4], "change")?,
        index: normal(parts[5], "index")?,
    })
}

pub fn describe_bip44_path(info: &Bip44PathInfo) -> String {
    let account = match info.account {
        0 => "first",
        1 => "second",
        2 => "third",
        3 => "fourth",
        4 => "fifth",
        _ => "later",
    };

    let chain = if info.change == 0 {
        "external receiving"
    } else {
        "change"
    };

    let address = match info.index {
        0 => "first",
        1 => "second",
        2 => "third",
        3 => "fourth",
        4 => "fifth",
        5 => "sixth",
        6 => "seventh",
        7 => "eighth",
        8 => "ninth",
        9 => "tenth",
        _ => "later",
    };

    format!(
        "purpose {} uses the {} account on the {} chain and selects the {} address",
        info.purpose, account, chain, address
    )
}

pub fn with_address_index(path: &str, new_index: u32) -> LabResult<String> {
    let info = decode_bip44_path(path)?;

    Ok(format!(
        "m/{}'/{}'/{}'/{} /{}",
        info.purpose, info.coin_type, info.account, info.change, new_index
    )
    .replace(" /", "/"))
}

pub fn derive_bip44_address(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    network: Network,
) -> LabResult<String> {
    decode_bip44_path(path)?;

    let seed_hex = crate::labs::lab07_bip39::mnemonic_seed_hex(mnemonic, passphrase)?;

    let seed = hex::decode(seed_hex)
        .map_err(|e| crate::LabError::Derivation(format!("invalid BIP39 seed: {e}")))?;

    let master = ExtendedPrivKey::new_master(network, &seed)
        .map_err(|e| crate::LabError::Derivation(format!("failed to create master key: {e}")))?;

    let derivation_path: DerivationPath = path
        .parse()
        .map_err(|e| crate::LabError::InvalidPath(format!("invalid derivation path: {e}")))?;

    let secp = bitcoin::secp256k1::Secp256k1::new();

    let child = master
        .derive_priv(&secp, &derivation_path)
        .map_err(|e| crate::LabError::Derivation(format!("failed to derive child key: {e}")))?;

    let public_key = PublicKey::new(child.private_key.public_key(&secp));

    let address = Address::p2pkh(&public_key, network);

    Ok(address.to_string())
}
