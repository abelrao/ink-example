#![cfg_attr(not(feature = "std"), no_std)]


use ink_lang as ink;


#[ink::contract]
mod advaita_health_contract {
    use ink_storage::{
        collections::HashMap as StorageMap,
        collections::vec::Vec as StorageVec,
        Lazy,
    };
    use ink_prelude::vec::Vec;
    use ink_prelude::string::String;
    use ink_lang::static_assertions::_core::borrow::Borrow;
    use ink_storage::traits::{PackedLayout, SpreadLayout, KeyPtr};
    use ink_primitives::Key;


    #[ink(storage)]
    pub struct AdvaitaHealthContract {
        questionnaires: StorageMap<(AccountId, u64), Survey>,
        indexes: StorageMap<AccountId, u64>,
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
            let quest_map: StorageMap<(AccountId, u64), Survey> = StorageMap::new();
            let indexes: StorageMap<AccountId, u64> = StorageMap::new();
            Self {
                questionnaires: quest_map,
                indexes,
            }
        }


        #[ink(message)]
        pub fn add_survey(&mut self, info: Survey) {
            let caller = Self::env().caller();
            let index = self.indexes.get(&caller).unwrap_or(&0);
            let new_index = index + 1;
            self.questionnaires.insert((caller.clone(), *index), info);
            self.indexes.insert(caller, new_index);
            Self::env().emit_event(Update {
                owner: caller,
            });
        }


        #[ink(message)]
        pub fn get_survey(&self, index: u64) -> Survey {
            let caller = Self::env().caller();
            let re = self.questionnaires.get(&(caller, index)).cloned().unwrap_or(Survey::default());
            Self::env().emit_event(Query {
                owner: caller,
            });
            return re;
        }
    }

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(
    Debug,
    PartialEq,
    Eq,
    scale_info::TypeInfo,
    ink_storage::traits::StorageLayout
    )
    )]
    pub struct Survey {
        pub content: String
    }

    impl Default for Survey {
        fn default() -> Self {
            Survey { content: String::new() }
        }
    }

    impl Clone for Survey {
        fn clone(&self) -> Self {
            Self {
                content: self.content.clone()
            }
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
