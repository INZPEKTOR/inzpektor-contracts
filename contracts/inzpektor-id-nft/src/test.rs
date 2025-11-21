#![cfg(test)]

extern crate std;

use soroban_sdk::{ testutils::Address as _, Address, Env, String };

use crate::contract::{ INZPEKTORID, INZPEKTORIDClient };

#[test]
fn initial_state() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_addr = env.register(INZPEKTORID, ());
    let client = INZPEKTORIDClient::new(&env, &contract_addr);

    let owner = Address::generate(&env);

    client.initialize(&owner);

    assert_eq!(client.name(), String::from_str(&env, "INZPEKTOR-ID"));
    assert_eq!(client.symbol(), String::from_str(&env, "IZK"));
}

#[test]
fn test_mint() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_addr = env.register(INZPEKTORID, ());
    let client = INZPEKTORIDClient::new(&env, &contract_addr);

    let owner = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(&owner);

    // Mint tokens sequentially
    client.mint(&user);
    client.mint(&user);

    // Verify ownership - sequential_mint starts at token_id 0
    assert_eq!(client.owner_of(&0), user);
    assert_eq!(client.owner_of(&1), user);

    // Verify balance
    assert_eq!(client.balance(&user), 2);

    // Verify total supply
    assert_eq!(client.total_supply(), 2);
}

// Add more tests bellow
