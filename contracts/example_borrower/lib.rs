#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod default_borrower {
    use flashloans::traits::borrower::*;
    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::vec::Vec;
    use openbrush::contracts::traits::psp22::PSP22Ref;
    use openbrush::traits::DefaultEnv;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct DefaultBorrower {}

    impl FlashloanBorrower for DefaultBorrower {
        /// Instantly returns `amount` + `fee` to provider. Normally, it would perform some 
        /// more complex logic to advantage the loan (see comments). 

        /// An important note is that we shouldn't transfer any tokens to this smart contract, 
        /// since they could be easilly withdrawn by any caller.
        #[ink(message)]
        fn on_flashloan(&mut self, provider: AccountId, token: AccountId, amount: u128, fee: u128) -> Result<(), FlashloanBorrowerError> {
            if PSP22Ref::balance_of(&token, Self::env().account_id()) < amount {
                return Err(FlashloanBorrowerError::FlashloanNotProvided);
            }

            // actual code would go there

            // transfer back
            let transfer_status = PSP22Ref::transfer(&token, provider, amount+fee, Vec::<u8>::new());
            if transfer_status.is_err() {
                return Err(FlashloanBorrowerError::ReturnToLenderFailed);
            }

            // In real applications, we'd transfer all earned assets to hardcoded owner address there,
            // so that none tokens of any kind are stored in the contract after flashloan.
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
