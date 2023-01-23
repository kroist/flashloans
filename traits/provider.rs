use openbrush::traits::AccountId;
use ink_prelude::string::String;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FlashloanProvidingError {
    Custom(String),
    TooLargeAmount,
    FlashloanNotReturned,
    TransferError,
    ReceiverIsNotCallable,
    ReceiverMethodFailed,
    CancelledByBorrower,
}


#[openbrush::trait_definition]
pub trait FlashloanProvider {
    ///  Gets fee for a `token` loan of a given `amount`, according to current Provider state.
    #[ink(message)]
    fn get_fee(&self, token: AccountId, amount: u128) -> u128;

    /// Maximal allowed loan for a given token. 
    /// Provider will accept all flashloans on amounts less than min(get_max_allowed_loan, token.ballance_of(self)).
    #[ink(message)]
    fn get_max_allowed_loan(&self, token: AccountId) -> u128;


    /// Performs some checks and then flashloans `amount` of `token` to `receiver` smart contract, bu calling on_flashloan
    /// function on `receiver` (which must implement `FlashloanBorrower` trait). At the end, it should check, if the receiver
    /// returned lent amount + a fee. 
    #[ink(message)]
    fn provide_flashloan(&mut self, receiver: AccountId, token: AccountId, amount: u128) -> Result<(), FlashloanProvidingError>;
}

#[openbrush::wrapper]
pub type FlashloanProviderRef = dyn FlashloanProvider;

