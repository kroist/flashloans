#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod default_provider {
    use flashloans::traits::provider::*;
    use flashloans::traits::borrower::FlashloanBorrowerRef;
    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::vec::Vec;
    use openbrush::contracts::traits::psp22::PSP22Ref;
    use openbrush::traits::DefaultEnv;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct DefaultProvider {
        fee_per_1M_tokens: u32
    }

    impl FlashloanProvider for DefaultProvider {
        #[ink(message)]
        fn get_fee(&self, _token: AccountId, amount: u128) -> u128 {
            //fee is rounded up
            (amount*(self.fee_per_1M_tokens as u128)+1_000_000-1)/1_000_000 
        }

        #[ink(message)]
        fn get_max_allowed_loan(&self, token: AccountId) -> u128 {
            PSP22Ref::balance_of(&token, Self::env().account_id())
        }

        #[ink(message)]
        fn provide_flashloan(&mut self, receiver: AccountId, token: AccountId, amount: u128) -> Result<(), FlashloanProvidingError> {
            if self.get_max_allowed_loan(token) < amount {
                return Err(FlashloanProvidingError::TooLargeAmount)
            }
            
            let fee = self.get_fee(token, amount);
            let expected_balance_after = fee + PSP22Ref::balance_of(&token, Self::env().account_id());

            let transfer_status = PSP22Ref::transfer(&token, receiver, amount, Vec::<u8>::new());
            if transfer_status.is_err() {
                return Err(FlashloanProvidingError::TransferError)
            }
            FlashloanBorrowerRef::on_flashloan(&receiver, token, Self::env().account_id(), amount, fee);
            
            if PSP22Ref::balance_of(&token, Self::env().account_id()) < expected_balance_after {
                return Err(FlashloanProvidingError::FlashloanNotReturned)
            }
            Ok(())
        }
    }

    impl DefaultProvider {
        #[ink(constructor)]
        pub fn new(fee_per_1M_tokens: u32) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self|{
                instance.fee_per_1M_tokens = fee_per_1M_tokens
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use ink_lang as ink;
    use crate::default_provider::DefaultProvider;
    use flashloans::traits::provider::*;
    use openbrush::test_utils::accounts;
    
    #[ink::test]
    fn get_fee_works() {
        let token_account_id = accounts().charlie;

        let provider = DefaultProvider::new(900); // 0.09%

        assert_eq!(provider.get_fee(token_account_id, 1000000), 900);
        assert_eq!(provider.get_fee(token_account_id, 10000), 9);
        assert_eq!(provider.get_fee(token_account_id, 20000), 18);
        assert_eq!(provider.get_fee(token_account_id, 20001), 19);
        assert_eq!(provider.get_fee(token_account_id, 1), 1);
        assert_eq!(provider.get_fee(token_account_id, 0), 0);
    }
    // other methods can't be tested, since `off-chain environment does not support contract invocation` 
}

