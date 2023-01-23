RELEASE_FLAGS = 

.PHONY: test clean

all: default_provider example_borrower psp22_token_a dex_arbitrage_borrower

test: test_default_provider test_psp22_token_a

clean: clean_default_provider clean_example_borrower clean_psp22_token_a clean_dex_arbitrage_borrower

default_provider:
	cd ./contracts/default_provider; cargo +nightly contract build --release $(RELEASE_FLAGS)

example_borrower:
	cd ./contracts/example_borrower; cargo +nightly contract build --release $(RELEASE_FLAGS)

psp22_token_a:
	cd ./contracts/psp22_token_a; cargo +nightly contract build --release $(RELEASE_FLAGS)

dex_arbitrage_borrower:
	cd ./contracts/dex_arbitrage_borrower; cargo +nightly contract build --release $(RELEASE_FLAGS)

test_default_provider:
	cd ./contracts/default_provider; cargo test

test_psp22_token_a:
	cd ./contracts/psp22_token_a; cargo test

clean_default_provider:
	cd ./contracts/default_provider; cargo clean

clean_example_borrower:
	cd ./contracts/example_borrower; cargo clean

clean_psp22_token_a:
	cd ./contracts/psp22_token_a; cargo clean

clean_dex_arbitrage_borrower:
	cd ./contracts/dex_arbitrage_borrower; cargo clean