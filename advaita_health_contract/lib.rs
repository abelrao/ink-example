#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod advaita_health_contract {
    use ink_storage::{
        collections::HashMap as StorageMap,
        collections::vec::Vec,
        Lazy,
    };

    use ink_lang::static_assertions::_core::borrow::Borrow;

    #[ink(storage)]
    pub struct AdvaitaHealthContract {
        alg_map: StorageMap<AccountId, u64>,

    }


    pub type Result<T> = core::result::Result<T, Error>;


    #[ink(event)]
    pub struct Update {
        #[ink(topic)]
        owner: AccountId,
    }

    #[ink(event)]
    pub struct Query {
        #[ink(topic)]
        owner: AccountId,
    }


    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Updated,
    }


    impl AdvaitaHealthContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let alg_map: StorageMap<AccountId, u64> = StorageMap::new();
            Self { alg_map }
        }


        #[ink(message)]
        pub fn update(&mut self, info: u64) {
            let caller = Self::env().caller();
            self.alg_map.insert(caller, info);
            Self::env().emit_event(Update {
                owner: caller,
            });
        }

        #[ink(message)]
        pub fn get_list(&self) -> u64 {
            let caller = Self::env().caller();
            let re = self.alg_map.get(&caller).unwrap_or(&(0 as u64));
            Self::env().emit_event(Query {
                owner: caller,
            });
            return *re;
        }
    }


    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let advaita_health_contract = AdvaitaHealthContract::new();
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {}
    }
}
