#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

mod provider;
use ink_lang as ink;

#[ink::contract]
mod default_provider {
    use crate::provider::FlashloanProvider;
    use ink_storage::traits::SpreadAllocate;

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
            panic!("TODO")
        }

        #[ink(message)]
        fn provide_flashloan(&self, receiver: ink_env::AccountId, token: AccountId, amount: u128) -> bool {
            panic!("TODO")
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
    use crate::provider::FlashloanProvider;
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
