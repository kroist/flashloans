#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
use ink_lang as ink;


#[openbrush::contract]
mod default_provider {
    use flashloans::traits::provider::*;
    use flashloans::traits::borrower::FlashloanBorrowerRef;
    use ink_storage::traits::SpreadAllocate;
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
        fn provide_flashloan(&self, receiver: AccountId, token: AccountId, amount: u128) -> Result<(), FlashloanProvidingError> {
            if self.get_max_allowed_loan(token) > amount {
                return Err(FlashloanProvidingError::TooLargeAmount)
            }
            
            let expected_fee = self.get_fee(token, amount);
            let expected_balance_after = expected_fee + PSP22Ref::balance_of(&token, Self::env().account_id());

            PSP22Ref::transfer(&token, receiver, amount, Vec::<u8>::new());
            FlashloanBorrowerRef::on_flashloan(&receiver, token, Self::env().account_id(), amount);
            
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
    use crate::default_provider::DefaultProvider;
    use flashloans::provider::FlashloanProvider;
    use ink_env::{test, DefaultEnvironment};
    use ink_lang as ink;

    #[ink::test]
    fn get_fee_works() {
        let token = test::default_accounts::<DefaultEnvironment>().eve;
        let provider = DefaultProvider::new(900); // 0.09%
        assert_eq!(provider.get_fee(token, 1000000), 900);
        assert_eq!(provider.get_fee(token, 10000), 9);
        assert_eq!(provider.get_fee(token, 20000), 18);
        assert_eq!(provider.get_fee(token, 20001), 19);
        assert_eq!(provider.get_fee(token, 1), 1);
        assert_eq!(provider.get_fee(token, 0), 0);
    }
}
