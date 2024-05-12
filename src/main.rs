extern crate dotenv;
use dotenv::dotenv;
use fuels::types::ContractId;
use fuels::{prelude::*, types::Bits256};
use std::env;
use std::str::FromStr;

mod contract_interfaces;

use crate::contract_interfaces::multisig_contract::info::{approval_weight, threshold};
// contracts:
/*
Contract target-contract:

Contract ID: 0xc75a615ab93775b879f43b43e9a98ed2287f6d149f0e5d234453bfea0b87e693
Deployed in block 0x6fb2f6a54188c3c19437ab978fb2ab97bbf1a994af3afb3e4e39b0864fad00cf

Contract multisig-contract:

Contract ID: 0xcaa9fe18ea5e314199d561f95bbe03e2d3c1b3676db3a378e40e97c8b4e09975
Deployed in block 0x4e7017d5dd5be84a40ce3c908026158eefbfa98793717a0c68f06d50973a30ea
*/

abigen!(
    Contract(name = "MultiSig", abi = "./abis/multisig-contract-abi.json"),
    Contract(
        name = "TargetContract",
        abi = "./abis/target-contract-abi.json"
    )
);
const MULTI_SIG_CONTRACT: &str =
    "0xcaa9fe18ea5e314199d561f95bbe03e2d3c1b3676db3a378e40e97c8b4e09975";
// const contract_id: ContractId =
//     ContractId::from_str("0xc75a615ab93775b879f43b43e9a98ed2287f6d149f0e5d234453bfea0b87e693"); //.expect("failed to create ContractId from string");

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok(); // Load .env file

    let phrase = env::var("MNEMONIC").expect("MNEMONIC must be set in .env");

    dbg!("getting the provider");
    let provider = Provider::connect("beta-5.fuel.network").await.unwrap();
    dbg!("have the provider");

    let wallet = WalletUnlocked::new_from_mnemonic_phrase(&phrase, Some(provider)).unwrap();

    // Get the wallet address. Used later with the faucet
    dbg!(wallet.address().to_string());
    // From a string.
    let multi_sig_contract_id =
        ContractId::from_str(MULTI_SIG_CONTRACT).expect("failed to create ContractId from string");

    // let addressStr: String = wallet.address().to_string();
    let address_bits: Bits256 = Bits256::from_hex_str(wallet.address().hash.to_string().as_str())
        .expect("unable to convert address");
    // dbg!(wallet.address().hash.to_string().as_str());
    let contract_instance = MultiSig::new(multi_sig_contract_id, wallet);

    dbg!("we are getting ther");
    let result = threshold(&contract_instance).await.value;

    dbg!(result);
    let deployer_approval_weight = approval_weight(&contract_instance, address_bits)
        .await
        .value;
    dbg!(deployer_approval_weight);

    // contract_instance.
    //     .get_owner()
    //     .await
    //     .expect("failed to get owner");

    Ok(())
}
