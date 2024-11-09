use dotenv::dotenv;
use ethers_rs::{
    types::{Address, H256},
    utils::hex,
};
use std::env;
use std::process::{exit, Command};
use std::str::FromStr;

fn main() {
    dotenv().ok();

    let mut private_key_deployer = env::var("PRIVATE_KEY_DEPLOYER").unwrap_or_else(|_| {
        eprintln!("PRIVATE_KEY_DEPLOYER is not set.");
        exit(1);
    });

    let mut private_key_syncer = env::var("PRIVATE_KEY_SYNCER").unwrap_or_else(|_| {
        eprintln!("PRIVATE_KEY_SYNCER is not set.");
        exit(1);
    });

    let deployer_address = compute_ethereum_address(&private_key_deployer).unwrap_or_else(|e| {
        eprintln!("Invalid PRIVATE_KEY_DEPLOYER: {}", e);
        exit(1);
    });

    let syncer_address = compute_ethereum_address(&private_key_syncer).unwrap_or_else(|e| {
        eprintln!("Invalid PRIVATE_KEY_SYNCER: {}", e);
        exit(1);
    });

    env::set_var("PRIVATE_KEY", private_key_deployer);

    let output = Command::new("othentic-cli")
        .args(&["network", "set-syncer", "--syncer-address", &syncer_address])
        .output()
        .unwrap_or_else(|e| {
            eprintln!("Failed to execute command: {}", e);
            exit(1);
        });

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn compute_ethereum_address(private_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let clean_key = private_key.strip_prefix("0x").unwrap_or(private_key);
    let private_key_bytes = H256::from_str(clean_key)?;
    let secp = secp256k1::Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(&private_key_bytes.as_bytes())?;
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
    let public_key_bytes = public_key.serialize_uncompressed();
    let hash = ethers_rs::utils::keccak256(&public_key_bytes[1..]);
    let address = Address::from_slice(&hash[12..]);
    Ok(format!("{:?}", address))
}
