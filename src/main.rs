use anyhow::{anyhow, Result};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

extern crate base64;
extern crate hex;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "fc_wallet_exporter",
    about = "Export Private Key from FileCoin Wallet"
)]
struct Cli {
    /// The path to the filecoin wallet file to read
    #[structopt(parse(from_os_str))]
    wallet_path: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::from_args();
    let encoded_wallet = fs::read_to_string(cli.wallet_path)?;
    println!("Encoded Wallet: {}", encoded_wallet);

    let decoded_bytes = hex::decode(&encoded_wallet)?;
    let value: Value = serde_json::from_slice(&decoded_bytes)?;
    println!("JSON: {}", value);

    // the private key is base64 encoded in little endian order
    if let Some(private_key_encoded) = value["PrivateKey"].as_str() {
        let mut pk = base64::decode(private_key_encoded)?;
        // reverse bytes to make it big endian
        pk.reverse();
        println!("Private Key (HEX): {}", hex::encode(pk));

        Ok(())
    } else {
        Err(anyhow!("PrivateKey in Json not found"))
    }
}
