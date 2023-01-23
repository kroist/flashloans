use openbrush::traits::AccountId;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FlashloanBorrowerError {
    ReturnToLenderFailed,
}

#[openbrush::trait_definition]
pub trait FlashloanBorrower {
    #[ink(message)]
    fn on_flashloan(
        &mut self, 
        provider: AccountId, 
        token: AccountId, 
        amount: u128,
        fee: u128
    ) -> Result<(), FlashloanBorrowerError>;
}

#[openbrush::wrapper]
pub type FlashloanBorrowerRef = dyn FlashloanBorrower;
