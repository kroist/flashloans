#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

use ink_lang as ink;

#[ink::trait_definition]
pub trait FlashloanBorrower {
    
    #[ink(message)]
    fn on_flashloan(
        &self, 
        provider: ink_env::AccountId, 
        token: ink_env::AccountId, 
        amount: u128
    ) -> bool;
}