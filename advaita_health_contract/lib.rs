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
    use ink_prelude::string::ToString;
    use ink_lang::static_assertions::_core::borrow::Borrow;
    use ink_storage::traits::{PackedLayout, SpreadLayout, KeyPtr};
    use ink_primitives::Key;


    #[ink(storage)]
    pub struct AdvaitaHealthContract {
        // todo need to multi contract,update table? consider dApp
        questionnaires: StorageMap<(AccountId, u64), Survey>,
        indexes: StorageMap<AccountId, u64>,
        tradition_questionnaires: StorageMap<(AccountId, u64), Survey>,
        tr_indexes: StorageMap<AccountId, u64>,
        prescription: StorageMap<(AccountId, u64), Prescription>,
        pr_indexes: StorageMap<AccountId, u64>,

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
            let tr_questionnaires: StorageMap<(AccountId, u64), Survey> = StorageMap::new();
            let tr_indexes: StorageMap<AccountId, u64> = StorageMap::new();
            let prescription: StorageMap<(AccountId, u64), Prescription> = StorageMap::new();
            let pr_indexes: StorageMap<AccountId, u64> = StorageMap::new();
            Self {
                questionnaires: quest_map,
                indexes,
                tradition_questionnaires: tr_questionnaires,
                tr_indexes,
                prescription,
                pr_indexes,
            }
        }


        #[ink(message)]
        pub fn add_modern_survey(&mut self, info: Survey) {
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
        pub fn get_modern_survey(&self, index: u64) -> Survey {
            let caller = Self::env().caller();
            let re = self.questionnaires.get(&(caller, index)).cloned().unwrap_or(Survey::default());
            Self::env().emit_event(Query {
                owner: caller,
            });
            return re;
        }

        #[ink(message)]
        pub fn modern_survey_list(&self) -> Vec<Survey> {
            let caller = Self::env().caller();
            let count = self.indexes.get(&caller).unwrap_or(&0);
            let mut vec: Vec<Survey> = Vec::new();
            for index in 0..(*count) {
                let survey = self.questionnaires.get(&(caller, index)).cloned().unwrap_or(Survey::default());
                vec.push(survey);
            }
            vec
        }

        #[ink(message)]
        pub fn add_tradition_survey(&mut self, info: Survey) {
            let caller = Self::env().caller();
            let index = self.tr_indexes.get(&caller).unwrap_or(&0);
            let new_index = index + 1;
            self.tradition_questionnaires.insert((caller.clone(), *index), info);
            self.tr_indexes.insert(caller, new_index);
            Self::env().emit_event(Update {
                owner: caller,
            });
        }


        #[ink(message)]
        pub fn get_tradition_survey(&self, index: u64) -> Survey {
            let caller = Self::env().caller();
            let re = self.tradition_questionnaires.get(&(caller, index)).cloned().unwrap_or(Survey::default());
            Self::env().emit_event(Query {
                owner: caller,
            });
            return re;
        }

        #[ink(message)]
        pub fn tradition_survey_list(&self) -> Vec<Survey> {
            let caller = Self::env().caller();
            let count = self.tr_indexes.get(&caller).unwrap_or(&0);
            let mut vec: Vec<Survey> = Vec::new();
            for index in 0..(*count) {
                let survey = self.tradition_questionnaires.get(&(caller, index)).cloned().unwrap_or(Survey::default());
                vec.push(survey);
            }
            vec
        }

        #[ink(message)]
        pub fn add_prescription(&mut self, info: Prescription) {
            let caller = Self::env().caller();
            let index = self.pr_indexes.get(&caller).unwrap_or(&0);
            let new_index = index + 1;
            self.prescription.insert((caller.clone(), *index), info);
            self.pr_indexes.insert(caller, new_index);
            Self::env().emit_event(Update {
                owner: caller,
            });
        }


        #[ink(message)]
        pub fn get_prescription(&self, index: u64) -> Prescription {
            let caller = Self::env().caller();
            let re = self.prescription.get(&(caller, index)).cloned().unwrap_or(Prescription::default());
            Self::env().emit_event(Query {
                owner: caller,
            });
            return re;
        }


        #[ink(message)]
        pub fn prescription_list(&self) -> Vec<Prescription> {
            let caller = Self::env().caller();
            let count = self.pr_indexes.get(&caller).unwrap_or(&0);
            let mut vec: Vec<Prescription> = Vec::new();
            for index in 0..(*count) {
                let survey = self.prescription.get(&(caller, index)).cloned().unwrap_or(Prescription::default());
                vec.push(survey);
            }
            vec
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
    pub struct Prescription {
        pub doctor_name: String,
        pub doctor_id: String,
        pub doctor_phone: String,
        pub patient_name: String,
        pub patient_phone: String,
        pub patient_id: String,
        pub content: String,
    }


    impl Default for Prescription {
        fn default() -> Self {
            Prescription {
                doctor_name: "".to_string(),
                doctor_id: "".to_string(),
                doctor_phone: "".to_string(),
                patient_name: "".to_string(),
                patient_phone: "".to_string(),
                patient_id: "".to_string(),
                content: "".to_string(),
            }
        }
    }

    impl Clone for Prescription {
        fn clone(&self) -> Self {
            Self {
                doctor_name: self.doctor_name.clone(),
                doctor_id: self.doctor_id.clone(),
                doctor_phone: self.doctor_phone.clone(),
                patient_name: self.patient_name.clone(),
                patient_phone: self.patient_phone.clone(),
                patient_id: self.patient_id.clone(),
                content: self.content.clone(),
            }
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn test_add_model_survey() {
            let mut advaita_health_contract = AdvaitaHealthContract::new();
            let sur = Survey { content: String::from("hello") };
            advaita_health_contract.add_modern_survey(sur.clone());
            let caller = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            let survey = advaita_health_contract.get_modern_survey(0);
            assert_eq!(sur, survey);
            let vec = advaita_health_contract.modern_survey_list();
            let mut result: Vec<Survey> = Vec::new();
            result.push(sur);
            assert_eq!(result, vec)
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {}
    }
}
