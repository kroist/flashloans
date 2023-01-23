all: default_provider example_borrower psp22_token_a

default_provider:
	cd ./contracts/default_provider; cargo +nightly contract build --release --optimization-passes=0

example_borrower:
	cd ./contracts/example_borrower; cargo +nightly contract build --release --optimization-passes=0

psp22_token_a:
	cd ./contracts/psp22_token_a; cargo +nightly contract build --release --optimization-passes=0
