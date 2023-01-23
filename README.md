[![Built with ink!](https://raw.githubusercontent.com/paritytech/ink/master/.images/badge.svg)](https://github.com/paritytech/ink)

## Development
Follow the [Aleph Zero guide](https://docs.alephzero.org/aleph-zero/build/installing-required-tools) to install rust and ink!. Note, that:
* probably you'll have to run `cargo install dylint-link` during the installation, when prompted so,
* You can now safely execute `cargo install cargo-contract` to install cargo-contract, instead of installing a specific snapshot as ntoted in the tutorial.

When adding new contract/library, make sure the `ink!` version used in `Cargo.toml` is `3.3`.
