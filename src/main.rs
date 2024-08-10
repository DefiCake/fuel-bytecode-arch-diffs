use fuel_tx::Salt;
use fuels::{
    prelude::{abigen, Contract},
    programs::contract::LoadConfiguration, types::Bits256,
};
use sha2::{Sha256, Digest};
use std::fs;

abigen!(Contract(
    name = "MyContract",
    abi = "out/release/my-fuel-project-abi.json",
));

const CONTRACT_BINARY: &str = "out/release/my-fuel-project.bin";
pub const DEFAULT_COIN_AMOUNT: u64 = 1_000_000_000;

#[tokio::main]
async fn main() {
    // Generate SHA256 hash of CONTRACT_BINARY
    let binary_content = fs::read(CONTRACT_BINARY).expect("Failed to read contract binary");
    let mut hasher = Sha256::new();
    hasher.update(&binary_content);
    let hash = hasher.finalize();
    
    println!("SHA256 of CONTRACT_BINARY: {:x}", hash);

    // Now the contract IDs
    let bridged_token_gateway: [u8; 32] = 
        hex::decode("0000000000000000000000005fc8d32690cc91d4c39d9d3abcbd16989f875707")
        .unwrap().try_into().unwrap();
    
    let configurables = MyContractConfigurables::default()
        .with_BRIDGED_TOKEN_GATEWAY(Bits256(bridged_token_gateway))
        .unwrap();

    let load_configuration = LoadConfiguration::default()
        .with_salt(Salt::zeroed())
        .with_configurables(configurables);

    let contract = Contract::load_from(CONTRACT_BINARY, load_configuration).unwrap();
    
    println!("Contract ID: {}", contract.contract_id());

}