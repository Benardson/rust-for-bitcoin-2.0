use serde_json::to_string_pretty;
use sha2::{Digest, Sha256};
use std::io::{Error, ErrorKind};

use transaction::{Amount, Input, Output, Transaction, Txid};

mod transaction;

fn read_u32(bytes: &mut &[u8]) -> Result<u32, Error> {
    if bytes.len() < 4 {
        return Err(Error::new(ErrorKind::UnexpectedEof, "not enough bytes"));
    }

    let value = u32::from_le_bytes(bytes[..4].try_into().unwrap());
    *bytes = &bytes[4..];

    Ok(value)
}

fn read_u64(bytes: &mut &[u8]) -> Result<u64, Error> {
    if bytes.len() < 8 {
        return Err(Error::new(ErrorKind::UnexpectedEof, "not enough bytes"));
    }

    let value = u64::from_le_bytes(bytes[..8].try_into().unwrap());
    *bytes = &bytes[8..];

    Ok(value)
}

fn read_amount(bytes: &mut &[u8]) -> Result<Amount, Error> {
    Ok(Amount::from_sat(read_u64(bytes)?))
}

fn read_compact_size(bytes: &mut &[u8]) -> Result<u64, Error> {
    if bytes.is_empty() {
        return Err(Error::new(ErrorKind::UnexpectedEof, "missing compact size"));
    }

    let prefix = bytes[0];
    *bytes = &bytes[1..];

    match prefix {
        0..=252 => Ok(prefix as u64),

        253 => {
            if bytes.len() < 2 {
                return Err(Error::new(ErrorKind::UnexpectedEof, "missing u16"));
            }

            let value = u16::from_le_bytes(bytes[..2].try_into().unwrap()) as u64;

            *bytes = &bytes[2..];

            Ok(value)
        }

        254 => {
            if bytes.len() < 4 {
                return Err(Error::new(ErrorKind::UnexpectedEof, "missing u32"));
            }

            let value = u32::from_le_bytes(bytes[..4].try_into().unwrap()) as u64;

            *bytes = &bytes[4..];

            Ok(value)
        }

        255 => {
            if bytes.len() < 8 {
                return Err(Error::new(ErrorKind::UnexpectedEof, "missing u64"));
            }

            let value = u64::from_le_bytes(bytes[..8].try_into().unwrap());

            *bytes = &bytes[8..];

            Ok(value)
        }
    }
}

fn read_txid(bytes: &mut &[u8]) -> Result<Txid, Error> {
    if bytes.len() < 32 {
        return Err(Error::new(ErrorKind::UnexpectedEof, "missing txid"));
    }

    let txid: [u8; 32] = bytes[..32].try_into().unwrap();

    *bytes = &bytes[32..];

    Ok(Txid::from_bytes(txid))
}

fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let hex = hex.trim();

    if hex.is_empty() {
        return Err("transaction hex is empty".into());
    }

    if !hex.len().is_multiple_of(2) {
        return Err("invalid hex length".into());
    }

    let mut bytes = Vec::with_capacity(hex.len() / 2);

    for i in (0..hex.len()).step_by(2) {
        bytes.push(u8::from_str_radix(&hex[i..i + 2], 16)?);
    }

    Ok(bytes)
}

fn hash_transaction(bytes: &[u8]) -> Txid {
    let first_hash = Sha256::digest(bytes);
    let second_hash = Sha256::digest(first_hash);

    let result: [u8; 32] = second_hash.into();

    Txid::from_bytes(result)
}

fn encode_compact_size(value: u64) -> Vec<u8> {
    match value {
        0..=252 => vec![value as u8],

        253..=65535 => {
            let mut result = vec![253];
            result.extend_from_slice(&(value as u16).to_le_bytes());
            result
        }

        65536..=4294967295 => {
            let mut result = vec![254];
            result.extend_from_slice(&(value as u32).to_le_bytes());
            result
        }

        _ => {
            let mut result = vec![255];
            result.extend_from_slice(&value.to_le_bytes());
            result
        }
    }
}

fn serialize_input(input: &Input) -> Vec<u8> {
    let mut bytes = Vec::new();

    bytes.extend_from_slice(&input.txid.0);
    bytes.extend_from_slice(&input.output_index.to_le_bytes());

    bytes.extend_from_slice(&encode_compact_size(input.script_sig.len() as u64));

    bytes.extend_from_slice(&input.script_sig);
    bytes.extend_from_slice(&input.sequence.to_le_bytes());

    bytes
}

fn serialize_output(output: &Output) -> Vec<u8> {
    let mut bytes = Vec::new();

    bytes.extend_from_slice(&output.amount.0.to_le_bytes());

    bytes.extend_from_slice(&encode_compact_size(output.script_pubkey.len() as u64));

    bytes.extend_from_slice(&output.script_pubkey);

    bytes
}

fn serialize_for_txid(
    version: u32,
    inputs: &[Input],
    outputs: &[Output],
    lock_time: u32,
) -> Vec<u8> {
    let mut bytes = Vec::new();

    bytes.extend_from_slice(&version.to_le_bytes());

    bytes.extend_from_slice(&encode_compact_size(inputs.len() as u64));

    for input in inputs {
        bytes.extend_from_slice(&serialize_input(input));
    }

    bytes.extend_from_slice(&encode_compact_size(outputs.len() as u64));

    for output in outputs {
        bytes.extend_from_slice(&serialize_output(output));
    }

    bytes.extend_from_slice(&lock_time.to_le_bytes());

    bytes
}

pub fn decode_transaction(transaction_hex: String) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = hex_to_bytes(&transaction_hex)?;
    let mut remaining: &[u8] = &bytes;

    let version = read_u32(&mut remaining)?;

    let segwit = remaining.len() >= 2 && remaining[0] == 0x00 && remaining[1] == 0x01;

    if segwit {
        remaining = &remaining[2..];
    }

    let input_count = read_compact_size(&mut remaining)? as usize;

    let mut inputs = Vec::with_capacity(input_count);

    for _ in 0..input_count {
        let txid = read_txid(&mut remaining)?;

        let output_index = read_u32(&mut remaining)?;

        let script_len = read_compact_size(&mut remaining)? as usize;

        if remaining.len() < script_len {
            return Err("invalid scriptSig length".into());
        }

        let script_sig = remaining[..script_len].to_vec();

        remaining = &remaining[script_len..];

        let sequence = read_u32(&mut remaining)?;

        inputs.push(Input {
            txid,
            output_index,
            script_sig,
            sequence,
        });
    }

    let output_count = read_compact_size(&mut remaining)? as usize;

    let mut outputs = Vec::with_capacity(output_count);

    for _ in 0..output_count {
        let amount = read_amount(&mut remaining)?;

        let script_len = read_compact_size(&mut remaining)? as usize;

        if remaining.len() < script_len {
            return Err("invalid scriptPubKey length".into());
        }

        let script_pubkey = remaining[..script_len].to_vec();

        remaining = &remaining[script_len..];

        outputs.push(Output {
            amount,
            script_pubkey,
        });
    }

    // SegWit witness data is not part of the TXID.
    if segwit {
        for _ in 0..input_count {
            let witness_count = read_compact_size(&mut remaining)? as usize;

            for _ in 0..witness_count {
                let witness_len = read_compact_size(&mut remaining)? as usize;

                if remaining.len() < witness_len {
                    return Err("invalid witness length".into());
                }

                remaining = &remaining[witness_len..];
            }
        }
    }

    let lock_time = read_u32(&mut remaining)?;

    if !remaining.is_empty() {
        return Err("unexpected bytes after locktime".into());
    }

    // TXID serialization excludes SegWit marker, flag and witness.
    let txid_bytes = serialize_for_txid(version, &inputs, &outputs, lock_time);

    let transaction_id = hash_transaction(&txid_bytes);

    let transaction = Transaction {
        transaction_id,
        version,
        inputs,
        outputs,
        lock_time,
    };

    Ok(to_string_pretty(&transaction)?)
}
