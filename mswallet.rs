#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub type Result<T> = core::result::Result<T, Error>;

type Balance = <ink_env::DefaultEnvironment as ink_env::Environment>::Balance;

pub enum Error {
    Invalid,
}

#[ink::contract]
mod multisig_wallet {
    use super::*;
    use ink_env::AccountId;
    use ink_prelude::vec::Vec;
    use ink_storage::{ //
        traits::{
            PackedLayout,
            SpreadAllocate,
            SpreadLayout,
        },
        Mapping,
    };
    use scale::{Encode, Decode, Output};
    
    pub(crate) enum Authority {
        Authorize(PermissionedAction),
        Action(ActionType, Engage),

    }
    pub(crate) enum Engage {
        Stamp,
        Vote,
        Write,
    }
    pub(crate) enum ActionType {
        Approve,
        Deny,
        Interact(Engage),
        Signal,
    }
    pub(crate) enum PermissionedAction {
        Mint,
        Transfer,
        Security,
    }
  
    pub(crate) struct Account {
        address: Hash,
        owners: Vec<AccountId>,
        balance: Balance,
        data: AccountId, // refers to outside object
    }

    #[ink(event)]
    pub struct Sign {
        #[ink(topic)]
        owner: AccountId,
        signature: Hash,
    }

    #[ink(event)]
    pub struct Approve {
        #[ink(topic)]
        item: Hash, // ?TODO: make more generic
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        recipient: Option<AccountId>,
        #[ink(topic)]
        witnesses: Option<Vec<AccountId>>,
    }

    #[ink(event)]
    pub struct Deny {
        #[ink(topic)]
        item: Hash,
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        recipient: Option<AccountId>,
        #[ink(topic)]
        witnesses: Option<Vec<AccountId>>,
    }

    #[ink(event)]
    pub struct Publish {
        #[ink(topic)]
        item: Hash,
        #[ink(topic)]
        authors: Vec<AccountId>,
        #[ink(topic)]
        recipient: Option<AccountId>,
    }

    #[ink(event)]
    pub struct Subscribe {
        #[ink(topic)]
        item: Hash, 
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        recipient: Option<AccountId>,
        #[ink(topic)]
        witnesses: Option<Vec<AccountId>>,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct MultisigWallet {
        owners: Mapping<AccountId, Hash>,  // K: DID , V: Signature
        escrow_balance: Option<Balance>,
        accounts: Vec<Account>,
    }

    impl MultisigWallet {
        
        #[ink(constructor)]
        pub fn new(
            proof: Hash,
            did_sigs: Mapping<AccountId, Hash>, // owners by DID, Signature
            escrow: Option<Balance>, 

        ) -> Self {
            // TODO: if number of dids != sigs then throw error
                ink_lang::utils::initialize_contract(|wallet: &mut Self| {
                    
                    wallet.owners = did_sigs.clone();
                    wallet.escrow_balance = escrow;
                    wallet.agents = Vec::from( 
                        // TODO: extract keys from Mapping
                    );
                    wallet.accounts = Vec::new();
                })
        }
        /// Alternative initialization option
        // #[ink(constructor)]
        // pub fn crypto() -> Self {
            
        // }

        #[ink(message)]
        fn get(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: TokenId,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<()>;
    
        #[ink(message)]
        fn create(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: TokenId,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<()>;
    
        #[ink(message)]
        fn read(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: TokenId,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<()>;
    
        #[ink(message)]
        fn update(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: TokenId,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<()>;
    
        #[ink(message)]
        fn delete(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: TokenId,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<()>;
        
    
    }
}