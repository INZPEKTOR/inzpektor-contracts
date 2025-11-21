#![cfg(test)]

extern crate std;

use soroban_sdk::{ Env, String };

use crate::contract::{ INZPEKTORID, INZPEKTORIDClient };

#[test]
fn initial_state() {
    let env = Env::default();

    let contract_addr = env.register(INZPEKTORID, ());
    let client = INZPEKTORIDClient::new(&env, &contract_addr);

    assert_eq!(client.name(), String::from_str(&env, "INZPEKTORID"));
}

// Add more tests bellow
