#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

use ink_lang as ink;

#[openbrush::contract]
mod default_borrower {
    use flashloans::traits::borrower::*;
    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::vec::Vec;
    use openbrush::contracts::traits::psp22::PSP22Ref;
    use ink_lang::codegen::Env;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct DefaultBorrower {}

    impl FlashloanBorrower for DefaultBorrower {
        /// Instantly returns `amount` + `fee` to provider. Normally, it would perform some 
        /// more complex logic to advantage the loan.
        #[ink(message)]
        fn on_flashloan(&mut self, provider: AccountId, token: AccountId, amount: u128, fee: u128) -> Result<(), FlashloanBorrowerError> {
            // actual code would go there

            // transfer back
            let transfer_status = PSP22Ref::transfer(&token, provider, amount+fee, Vec::<u8>::new());
            if transfer_status.is_err() {
                return Err(FlashloanBorrowerError::ReturnToLenderFailed);
            }
            Ok(())
        }
    }

    impl DefaultBorrower {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self|{})
        }
    }
}
