#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;


#[ink::contract]
mod factory {
    use crate::TokenId;
    use crate::TokenType;
    use mswallet::multisig_wallet;

    #[ink(event)]
    pub struct Token {
        #[ink(topic)]
        id: TokenId, 
        owner: AccountId,
        style: TokenType,
    }

    #[ink(storage)]
    pub struct Factory {
        coin_total_supply: u64,
    }

    impl Factory {
        
        #[ink(constructor)]
        pub fn new(proof: Hash, init_supply: u64) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                
                contract.coin_total_supply = init_supply;
            })
        }

        #[ink(message)]
        fn mint(
            &mut self,
            style: TokenType,
            load: Params,
        ) -> Result<Token> {

        }
    }
} 