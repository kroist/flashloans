#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

use ink_lang as ink;

#[openbrush::contract]
mod default_borrower {
    use flashloans::traits::borrower::*;
    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::vec::Vec;
    use openbrush::contracts::traits::psp22::PSP22Ref;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct DefaultBorrower {}

    impl FlashloanBorrower for DefaultBorrower {
        #[ink(message)]
        fn on_flashloan(&self, provider: AccountId, token: AccountId, amount: u128, fee: u128) -> bool {
            // if self.env().caller() != TODO {return false}

            // actual code

            let transfer_status = PSP22Ref::transfer(&token, provider, amount+fee, Vec::<u8>::new());
            if transfer_status.is_err() {
                return false;
            }
            return true;
        }
    }

    impl DefaultBorrower {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self|{})
        }
    }
}
