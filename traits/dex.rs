use openbrush::traits::AccountId;


#[openbrush::wrapper]
pub type PairRef = dyn Pair;

#[openbrush::trait_definition]
pub trait Pair {
    // Function, that will be called by FlashloanProvider after it lends some tokens ot FlashloanBorrower.
    // At the end, it must return `amount` + `fee` `tokens` to `provider`.
    #[ink(message)]
    fn swap_token_with_token(
        &mut self,
        token1: AccountId, 
        token2: AccountId, 
        amount: u128,
        price: u128,
        slippage: u128
    ) -> u128;
}
