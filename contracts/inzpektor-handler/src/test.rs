#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Bytes, Env, String};

// Mock contracts for testing
mod mock_verifier {
    use soroban_sdk::{contract, contractimpl, Bytes, BytesN, Env};

    #[contract]
    pub struct MockVerifier;

    #[contractimpl]
    impl MockVerifier {
        pub fn verify_proof(e: Env, _vk_json: Bytes, _proof_blob: Bytes) -> BytesN<32> {
            // Return a dummy proof ID for testing
            BytesN::from_array(&e, &[1u8; 32])
        }
    }
}

mod mock_nft {
    use soroban_sdk::{contract, contractimpl, testutils::Address as _, Address, Env, String};

    #[contract]
    pub struct MockNFT;

    #[contractimpl]
    impl MockNFT {
        pub fn mint(_e: Env, _to: Address) {
            // Mock mint - does nothing
        }

        pub fn balance(_e: Env, _account: Address) -> u32 {
            1
        }

        pub fn owner_of(e: Env, _token_id: u32) -> Address {
            Address::generate(&e)
        }

        pub fn name(e: Env) -> String {
            String::from_str(&e, "INZPEKTOR-ID")
        }

        pub fn symbol(e: Env) -> String {
            String::from_str(&e, "IZK")
        }

        pub fn base_uri(e: Env) -> String {
            String::from_str(&e, "https://inzpektor.io/nft/")
        }
    }
}

// Helper function to initialize using the client
fn setup_contract_storage(client: &InzpektorHandlerContractClient, admin: &Address, verifier: &Address, nft: &Address) {
    client.initialize(admin, verifier, nft);
}

#[test]
fn test_constructor() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let verifier_contract = env.register(mock_verifier::MockVerifier, ());
    let nft_contract = env.register(mock_nft::MockNFT, ());

    let contract_id = env.register(InzpektorHandlerContract, ());
    let client = InzpektorHandlerContractClient::new(&env, &contract_id);

    setup_contract_storage(&client, &admin, &verifier_contract, &nft_contract);

    // Verify stored addresses
    assert_eq!(client.get_admin(), admin);
    assert_eq!(client.get_verifier_contract(), verifier_contract);
    assert_eq!(client.get_nft_contract(), nft_contract);
}

#[test]
fn test_mint_inzpektor_id() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let verifier_contract = env.register(mock_verifier::MockVerifier, ());
    let nft_contract = env.register(mock_nft::MockNFT, ());

    let contract_id = env.register(InzpektorHandlerContract, ());
    let client = InzpektorHandlerContractClient::new(&env, &contract_id);

    setup_contract_storage(&client, &admin, &verifier_contract, &nft_contract);

    let vk_json = Bytes::from_slice(&env, b"mock_vk");
    let proof_blob = Bytes::from_slice(&env, b"mock_proof");

    let result = client.mint_inzpektor_id(&user, &vk_json, &proof_blob);
    assert_eq!(result, String::from_str(&env, "Minted INZPEKTOR-ID NFT successfully"));
}

#[test]
fn test_get_nft_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let verifier_contract = env.register(mock_verifier::MockVerifier, ());
    let nft_contract = env.register(mock_nft::MockNFT, ());

    let contract_id = env.register(InzpektorHandlerContract, ());
    let client = InzpektorHandlerContractClient::new(&env, &contract_id);

    setup_contract_storage(&client, &admin, &verifier_contract, &nft_contract);

    let balance = client.get_nft_balance(&user);
    assert_eq!(balance, 1);
}

#[test]
fn test_get_nft_metadata() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let verifier_contract = env.register(mock_verifier::MockVerifier, ());
    let nft_contract = env.register(mock_nft::MockNFT, ());

    let contract_id = env.register(InzpektorHandlerContract, ());
    let client = InzpektorHandlerContractClient::new(&env, &contract_id);

    setup_contract_storage(&client, &admin, &verifier_contract, &nft_contract);

    let (name, symbol, base_uri) = client.get_nft_metadata();
    assert_eq!(name, String::from_str(&env, "INZPEKTOR-ID"));
    assert_eq!(symbol, String::from_str(&env, "IZK"));
    assert_eq!(base_uri, String::from_str(&env, "https://inzpektor.io/nft/"));
}

#[test]
fn test_get_nft_owner() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let verifier_contract = env.register(mock_verifier::MockVerifier, ());
    let nft_contract = env.register(mock_nft::MockNFT, ());

    let contract_id = env.register(InzpektorHandlerContract, ());
    let client = InzpektorHandlerContractClient::new(&env, &contract_id);

    setup_contract_storage(&client, &admin, &verifier_contract, &nft_contract);

    let token_id: u32 = 1;
    let owner = client.get_nft_owner(&token_id);

    // Just verify it returns an address without panicking
    assert!(owner.to_string().len() > 0);
}

#[test]
fn test_getters() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let verifier_contract = env.register(mock_verifier::MockVerifier, ());
    let nft_contract = env.register(mock_nft::MockNFT, ());

    let contract_id = env.register(InzpektorHandlerContract, ());
    let client = InzpektorHandlerContractClient::new(&env, &contract_id);

    setup_contract_storage(&client, &admin, &verifier_contract, &nft_contract);

    // Test all getters
    assert_eq!(client.get_admin(), admin);
    assert_eq!(client.get_verifier_contract(), verifier_contract);
    assert_eq!(client.get_nft_contract(), nft_contract);
}
