use openbrush::traits::AccountId;
use ink_prelude::string::String;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FlashloanBorrowerError {
    Custom(String),
    FlashloanNotProvided,
    ReturnToLenderFailed,
}

#[openbrush::trait_definition]
pub trait FlashloanBorrower {
    // Function, that will be called by FlashloanProvider after it lends some tokens ot FlashloanBorrower.
    // At the end, it must return `amount` + `fee` `tokens` to `provider`.
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
