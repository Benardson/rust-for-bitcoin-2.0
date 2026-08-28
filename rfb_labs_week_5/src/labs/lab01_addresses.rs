use bitcoin::Network;
use std::str::FromStr;

use crate::model::{AddressFormat, AddressReport};
use crate::{LabError, LabResult};

pub fn identify_prefix(address: &str) -> AddressFormat {
    if address.starts_with('1') || address.starts_with('m') || address.starts_with('n') {
        AddressFormat::P2pkh
    } else if address.starts_with('3') || address.starts_with('2') {
        AddressFormat::P2sh
    } else if address.starts_with("bc1q")
        || address.starts_with("tb1q")
        || address.starts_with("bcrt1q")
    {
        AddressFormat::P2wpkh
    } else if address.starts_with("bc1p")
        || address.starts_with("tb1p")
        || address.starts_with("bcrt1p")
    {
        AddressFormat::P2tr
    } else {
        AddressFormat::Unknown
    }
}

pub fn expected_prefix(format: AddressFormat, network: Network) -> Option<&'static str> {
    match (format, network) {
        (AddressFormat::P2pkh, Network::Bitcoin) => Some("1"),
        (AddressFormat::P2sh, Network::Bitcoin) => Some("3"),
        (AddressFormat::P2wpkh, Network::Bitcoin) => Some("bc1q"),
        (AddressFormat::P2tr, Network::Bitcoin) => Some("bc1p"),

        (AddressFormat::P2pkh, Network::Testnet)
        | (AddressFormat::P2pkh, Network::Signet)
        | (AddressFormat::P2pkh, Network::Regtest)
        | (AddressFormat::P2pkh, Network::Testnet4) => Some("m/n"),

        (AddressFormat::P2sh, Network::Testnet)
        | (AddressFormat::P2sh, Network::Signet)
        | (AddressFormat::P2sh, Network::Regtest)
        | (AddressFormat::P2sh, Network::Testnet4) => Some("2"),

        (AddressFormat::P2wpkh, Network::Testnet) | (AddressFormat::P2wpkh, Network::Signet) => {
            Some("tb1q")
        }

        (AddressFormat::P2tr, Network::Testnet) | (AddressFormat::P2tr, Network::Signet) => {
            Some("tb1p")
        }

        (AddressFormat::P2wpkh, Network::Regtest) => Some("bcrt1q"),
        (AddressFormat::P2tr, Network::Regtest) => Some("bcrt1p"),

        (AddressFormat::P2wpkh, Network::Testnet4) => Some("tb1q"),
        (AddressFormat::P2tr, Network::Testnet4) => Some("tb1p"),

        (AddressFormat::Unknown, _) => None,
    }
}

pub fn inspect_address(address: &str, network: Network) -> LabResult<AddressReport> {
    let address =
        bitcoin::Address::from_str(address).map_err(|e| LabError::InvalidAddress(e.to_string()))?;

    let address = address
        .require_network(network)
        .map_err(|e| LabError::WrongNetwork(e.to_string()))?;

    let format = match address.address_type() {
        Some(bitcoin::AddressType::P2pkh) => AddressFormat::P2pkh,
        Some(bitcoin::AddressType::P2sh) => AddressFormat::P2sh,
        Some(bitcoin::AddressType::P2wpkh) => AddressFormat::P2wpkh,
        Some(bitcoin::AddressType::P2tr) => AddressFormat::P2tr,
        _ => AddressFormat::Unknown,
    };

    Ok(AddressReport {
        address: address.to_string(),
        network: network.to_string(),
        format,
        script_pubkey_hex: address.script_pubkey().to_hex_string(),
    })
}

pub fn script_pubkey_hex(address: &str, network: Network) -> LabResult<String> {
    Ok(inspect_address(address, network)?.script_pubkey_hex)
}
