#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod dao_council {
    use super::*;
    use mswallet::multisig_wallet;
    use mint;
}
