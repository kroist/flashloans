# Flash loans
[![Built with ink!](https://raw.githubusercontent.com/paritytech/ink/master/.images/badge.svg)](https://github.com/paritytech/ink)

## Development
Follow the [Aleph Zero guide](https://docs.alephzero.org/aleph-zero/build/installing-required-tools) to install rust and ink!. Note, that:
* probably you'll have to run `cargo install dylint-link` during the installation, when prompted so,
* You can now safely execute `cargo install cargo-contract` to install cargo-contract, instead of installing a specific snapshot as ntoted in the tutorial.

When adding new contract/library, make sure the `ink!` version used in `Cargo.toml` is `3.3`.

## Unit testing
There are only a few unit tests, because cross-contract unit testing is not available at the current version of ink!, and PSP22 token transfers calls by standard specification __before_transaction_ method on receiver. 

To run these tests, type `make test` in console.

## Deployment to Aleph Zero Testnet:
1. `make all` or `make all-unoptimized`.
2. Open [online blockchain frontend](https://azero.dev/?rpc=wss%3A%2F%2Fws-smartnet.test.azero.dev#/explorer) and login to your account.
3. Make sure you have enough `SZERO` tokens (or get them on https://faucet-smartnet.test.azero.dev/).
4. Upload & deploy these contracts: _contracts/example_borrower_ and _contracts/psp22_token_a_. For _contracts/default_provider_ set `fee_per_1M_tokens` to, for example, 900 (correcponding to 0.09% fee).

## Manual tests on chain
5. Mint some (e.g 1'000'000) "_PSP22 token A_" tokens to provider, and some less to borrower (e.g 10).
6. Execute _provide_flashloan_ method on _contracts/default_provider_, setting receiver to _exaple_borrower_, token to _PSP22 token A_ and amount to some integer.
7. Transaction should be successfull and balances of both accounts should be equal to previous state +- flash loan fee.
