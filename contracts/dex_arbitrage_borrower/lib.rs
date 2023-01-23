#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod dex_arbitrage_borrower {
    use flashloans::traits::borrower::*;
    use flashloans::traits::provider::FlashloanProviderRef;
    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::vec::Vec;
    use openbrush::contracts::traits::psp22::PSP22Ref;
    use openbrush::traits::DefaultEnv;
    use flashloans::traits::dex::PairRef;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct DexArbitrageBorrower {
        provider: AccountId,
        token1: AccountId, 
        token2: AccountId, 
        price1: u128,
        price2: u128,
        fee_token: AccountId, 
        fee_amount: u128, 
        slippage: u128,
        dex1: AccountId,
        dex2: AccountId
    }

    impl FlashloanBorrower for DexArbitrageBorrower {
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

            let swapped_amount = PairRef::swap_token_with_token(&self.dex1, token, self.token2, amount, self.price1, self.slippage);
            PairRef::swap_token_with_token(&self.dex2, self.token2, token, swapped_amount, self.price2, self.slippage);

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

    impl DexArbitrageBorrower {

        #[ink(message)]
        pub fn execute_swap(&mut self) {

            let max_loan = FlashloanProviderRef::get_max_allowed_loan(&self.provider, self.token1);

            FlashloanProviderRef::provide_flashloan(&self.provider, self.env().account_id(), self.token1, max_loan);

        }

        #[ink(constructor)]
        pub fn new(
            provider: AccountId,
            token1: AccountId, 
            token2: AccountId,
            price1: u128,
            price2: u128,
            fee_token: AccountId, 
            fee_amount: u128, 
            slippage: u128,
            dex1: AccountId,
            dex2: AccountId
        ) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self|{
                instance.provider = provider;
                instance.token1 = token1;
                instance.token2 = token2;
                instance.price1 = price1;
                instance.price2 = price2;
                instance.fee_token = fee_token;
                instance.fee_amount = fee_amount;
                instance.slippage = slippage;
                instance.dex1 = dex1;
                instance.dex2 = dex2;
            })
        }
    }
}
