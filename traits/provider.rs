use openbrush::traits::AccountId;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FlashloanProvidingError {
    TooLargeAmount,
    FlashloanNotReturned,
    TransferError
}

#[openbrush::trait_definition]
pub trait FlashloanProvider {
    #[ink(message)]
    fn get_fee(&self, token: AccountId, amount: u128) -> u128;

    #[ink(message)]
    fn get_max_allowed_loan(&self, token: AccountId) -> u128;

    #[ink(message)]
    fn provide_flashloan(&self, receiver: AccountId, token: AccountId, amount: u128) -> Result<(), FlashloanProvidingError>;
}

#[openbrush::wrapper]
pub type FlashloanProviderRef = dyn FlashloanProvider;

