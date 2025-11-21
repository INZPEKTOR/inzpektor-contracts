#![no_std]

use soroban_sdk::{Address, Bytes, BytesN, Env, IntoVal, String, Symbol, contract, contractimpl, contracttype, vec};

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

    pub fn mint_inzpektor_id(e: Env, user: Address, _nft_expires_at: u64, vk_json: Bytes, proof_blob: Bytes) -> String {
      let actual_admin: Address = e.storage().instance().get(&DataKey::Admin).expect("admin not set");
      actual_admin.require_auth();

      // Verify proof by calling the verifier contract
      let verifier_contract_address: Address = e.storage().instance().get(&DataKey::ZKVerifierContract).expect("verifier not set");

      // Call verify_proof on the ultrahonk verifier contract
      let verify_fn = Symbol::new(&e, "verify_proof");
      let _proof_id: BytesN<32> = e.invoke_contract(
          &verifier_contract_address,
          &verify_fn,
          vec![&e, vk_json.into_val(&e), proof_blob.into_val(&e)]
      );

      // Proof verified successfully, mint INZPEKTOR-ID NFT
      let inzpektor_id_contract_address: Address = e.storage().instance().get(&DataKey::InzpektorIDNFTContract).expect("INZPEKTOR-ID contract not set");

      // Call mint on the NFT contract
      let mint_fn = Symbol::new(&e, "mint");
      let token_id: u32 = 1; // In practice, this should be unique
      let _: () = e.invoke_contract(
          &inzpektor_id_contract_address,
          &mint_fn,
          vec![&e, user.into_val(&e), token_id.into_val(&e)]
      );

      String::from_str(&e, "Minted INZPEKTOR-ID NFT successfully")
    }
}

mod test;
