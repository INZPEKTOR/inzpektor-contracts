// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.4.1


use soroban_sdk::{Address, Env, String, contract, contractimpl};
use stellar_access::ownable::{self as ownable};
use stellar_macros::{default_impl, only_owner};
use stellar_tokens::non_fungible::{Base, NonFungibleToken, enumerable::{NonFungibleEnumerable, Enumerable}};

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
    }

    #[only_owner]
    pub fn mint(e: &Env, to: Address) {
        Enumerable::sequential_mint(e, &to);
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
