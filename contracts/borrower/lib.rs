#![cfg_attr(not(feature = "std"), no_std)]

mod borrower;
use ink_lang as ink;

#[ink::contract]
mod default_borrower {
    use crate::borrower::FlashloanBorrower;
    use ink_storage::traits::SpreadAllocate;


    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct DefaultBorrower {}

    impl FlashloanBorrower for DefaultBorrower {
        #[ink(message)]
        fn on_flashloan(&self, provider: ink_env::AccountId, token: AccountId, amount: u128) -> bool {
            panic!("TODO")
        }
    }

    impl DefaultBorrower {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self|{})
        }
    }
}
