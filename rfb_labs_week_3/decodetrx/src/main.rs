use clap::{Arg, Command};
use decodetrx::decode_transaction;

fn main() {
    let matches = Command::new("decodetrx")
        .about("Decode a raw Bitcoin transaction")
        .arg(
            Arg::new("transaction")
                .help("Raw Bitcoin transaction in hexadecimal")
                .required(true),
        )
        .get_matches();

    let transaction_hex = matches
        .get_one::<String>("transaction")
        .unwrap();

    match decode_transaction(transaction_hex.to_string()) {
        Ok(decoded) => println!("{}", decoded),
        Err(error) => eprintln!("Error decoding transaction: {}", error),
    }
}
