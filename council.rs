#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod dao_council {

    use ink_env::AccountId;
    use ink_prelude::vec::Vec;
    use mswallet::multisig_wallet;
    
    #[ink(event)]
    pub struct Decision {
        #[ink(topic)]
        item: AccountId, 
        #[ink(topic)]
        outcome: Hash,
    }

    #[ink(storage)]
    pub struct Council {
        members: Vec<AccountId>,
    }

    impl Council {
        
        #[ink(constructor)]
        pub fn new(
            proof: Hash,
            members: Vec<AccountId>,

        ) -> Self {
                ink_lang::utils::initialize_contract(|contract: &mut Self| {
                    
                    contract.members = members;
                })
        }

        #[ink(message)]
        fn vote(
            &mut self,
            signature: Hash,
        ) -> Result<()>;
    }
}