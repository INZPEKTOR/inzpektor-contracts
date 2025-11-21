// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.4.1


use soroban_sdk::{Address, Env, String, contract, contractimpl, contracttype, Map};
use stellar_access::ownable::{self as ownable};
use stellar_macros::{default_impl, only_owner};
use stellar_tokens::non_fungible::{Base, NonFungibleToken, enumerable::{NonFungibleEnumerable, Enumerable}};

#[contracttype]
pub enum DataKey {
    Expirations, // Map<u32, u64> - token_id -> expiration_timestamp
}

#[contract]
pub struct INZPEKTORID;

#[contractimpl]
impl INZPEKTORID {
    pub fn initialize(e: &Env, owner: Address) {
        let uri = String::from_str(e, "https://www.inzpektor.com/ids/");
        let name = String::from_str(&e, "INZPEKTOR-ID");
        let symbol = String::from_str(&e, "IZK");
        Base::set_metadata(&e, uri, name, symbol);
        ownable::set_owner(e, &owner);

        // Initialize empty expirations map
        let expirations: Map<u32, u64> = Map::new(&e);
        e.storage().instance().set(&DataKey::Expirations, &expirations);
    }

    #[only_owner]
    pub fn mint(e: &Env, to: Address, expires_at: u64) -> u32 {
        // Get current supply to know what token_id will be minted
        let token_id = Enumerable::total_supply(e);

        // Mint the token
        Enumerable::sequential_mint(e, &to);

        // Store the expiration timestamp for this specific token
        let mut expirations: Map<u32, u64> = e.storage()
            .instance()
            .get(&DataKey::Expirations)
            .unwrap_or(Map::new(&e));
        expirations.set(token_id, expires_at);
        e.storage().instance().set(&DataKey::Expirations, &expirations);

        token_id
    }

    /// Get the expiration timestamp for a specific token
    pub fn get_expiration(e: Env, token_id: u32) -> u64 {
        let expirations: Map<u32, u64> = e.storage()
            .instance()
            .get(&DataKey::Expirations)
            .unwrap_or(Map::new(&e));
        expirations.get(token_id).unwrap_or(0)
    }

    /// Check if a token is expired
    pub fn is_expired(e: Env, token_id: u32) -> bool {
        let expiration = Self::get_expiration(e.clone(), token_id);
        if expiration == 0 {
            return false; // No expiration set
        }
        e.ledger().timestamp() > expiration
    }
}

#[default_impl]
#[contractimpl]
impl NonFungibleToken for INZPEKTORID {
    type ContractType = Enumerable;

}

//
// Extensions
//

#[default_impl]
#[contractimpl]
impl NonFungibleEnumerable for INZPEKTORID {}
