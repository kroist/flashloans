#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod psp22_token_a {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::Storage;
    use openbrush::contracts::psp22::extensions::burnable::*;
    use openbrush::contracts::psp22::extensions::mintable::*;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct PSP22TokenA {
        #[storage_field]
        psp22: psp22::Data,
    }
    
    impl PSP22 for PSP22TokenA {}
    impl PSP22Burnable for PSP22TokenA {}
    impl PSP22Mintable for PSP22TokenA {}
        
    impl PSP22TokenA {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self|{
                _instance._mint(_instance.env().caller(), initial_supply).expect("Should mint"); 
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::psp22_token_a::PSP22TokenA;
    use ink_env::{test, DefaultEnvironment};
    use ink_lang as ink;
    use openbrush::contracts::psp22::*;
    use openbrush::test_utils::accounts;

    #[ink::test]
    fn balance_of_works() {
        test::set_caller::<DefaultEnvironment>(accounts().alice);
        let mytoken = PSP22TokenA::new(1000);
        assert_eq!(mytoken.balance_of(accounts().alice), 1000);
        assert_eq!(mytoken.balance_of(accounts().bob), 0);
    }
}