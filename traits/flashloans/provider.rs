use ink_lang as ink;

#[ink::trait_definition]
pub trait FlashloanProvider {
    #[ink(message)]
    fn get_fee(&self, token: ink_env::AccountId, amount: u128) -> u128;

    #[ink(message)]
    fn get_max_allowed_loan(&self, token: ink_env::AccountId) -> u128;

    #[ink(message)]
    fn provide_flashloan(&self, receiver: ink_env::AccountId, token: ink_env::AccountId, amount: u128) -> bool;
}