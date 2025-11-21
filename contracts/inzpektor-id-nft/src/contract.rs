// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.4.1


use soroban_sdk::{contract, contractimpl, Env, String};
use stellar_macros::default_impl;
use stellar_tokens::non_fungible::{Base, NonFungibleToken};

#[contract]
pub struct INZPEKTORID;

#[contractimpl]
impl INZPEKTORID {
    pub fn __constructor(e: &Env) {
        let uri = String::from_str(e, "");
        let name = String::from_str(e, "INZPEKTOR-ID");
        let symbol = String::from_str(e, "IZK");
        Base::set_metadata(e, uri, name, symbol);
    }
}

#[default_impl]
#[contractimpl]
impl NonFungibleToken for INZPEKTORID {
    type ContractType = Base;

}
