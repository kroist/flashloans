use openbrush::traits::AccountId;

#[openbrush::trait_definition]
pub trait FlashloanBorrower {
    #[ink(message)]
    fn on_flashloan(
        &self, 
        provider: AccountId, 
        token: AccountId, 
        amount: u128,
        fee: u128
    ) -> bool;
}

#[openbrush::wrapper]
pub type FlashloanBorrowerRef = dyn FlashloanBorrower;
