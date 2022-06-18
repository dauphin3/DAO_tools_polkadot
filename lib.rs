#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub type TokenId = u128;

pub enum TokenType {
    Coin,
    Nft,
    Auth,
    Did,
    Claim,
}

pub enum Params {
    
}