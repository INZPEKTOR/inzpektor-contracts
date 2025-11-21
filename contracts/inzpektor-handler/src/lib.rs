#![no_std]

use soroban_sdk::{Address, Bytes, Env, String, Vec, contract, contractimpl, contracttype, vec};

mod ultrahonk_contract {
    soroban_sdk::contractimport!(file = "ultrahonk_zk.wasm");
}

mod inzpektor_id_nft {
    soroban_sdk::contractimport!(file = "inzpektor_id_nft.wasm");
}

// Create a DataKey type for storing admin and contract addresses
#[contracttype]
enum DataKey {
    Admin,
    ZKVerifierContract,
    InzpektorIDNFTContract,
}


#[contract]
pub struct InzpektorHandlerContract;

#[contractimpl]
impl InzpektorHandlerContract {
    pub fn __constructor(e: &Env, admin: Address, verifier_contract: Address, inzpektor_id_contract: Address) {
        admin.require_auth();
        // Initialization logic can be added here if needed
        e.storage().instance().set(&DataKey::Admin, &admin);
        e.storage().instance().set(&DataKey::ZKVerifierContract, &verifier_contract);
        e.storage().instance().set(&DataKey::InzpektorIDNFTContract, &inzpektor_id_contract);
    }

    pub fn mint_inzpektor_id(e: &Env, user: Address, nft_expires_at: u64, vk_json: Bytes, proof_blob: Bytes) -> Vec<String> {
      let actual_admin: Address = e.storage().instance().get(&DataKey::Admin).expect("admin not set");
      actual_admin.require_auth();

      // Verify proof
      let verifier_contract_address: Address = e.storage().instance().get(&DataKey::ZKVerifierContract).expect("verifier not set");
      let ultrahonk_client = ultrahonk_contract::Client::new(e, &verifier_contract_address);

      match ultrahonk_client.verify_proof(&vk_json, &proof_blob) {
          Ok(Ok(result)) => {
              // Mint INZPEKTOR-ID NFT
              let inzpektor_id_contract_address: Address = e.storage().instance().get(&DataKey::InzpektorIDNFTContract).expect("INZPEKTOR-ID contract not set");
              let inzpektor_id_client = inzpektor_id_nft::Client::new(e, &inzpektor_id_contract_address);

              // For simplicity, using a fixed token_id; in practice, this should be unique
              let token_id: u32 = 1;
              inzpektor_id_client.mint(e, user.clone(), token_id);

              vec![e, String::from_str(e, "Minted INZPEKTOR-ID NFT successfully")]
          },
          _ => {
              vec![e, String::from_str(e, "Proof verification failed")]
          }
      }


    }
}

mod test;
