_ := $(shell bash pwn.sh)
.PHONY: build run test clean fmt fmt-check clippy taplo taplo-check deny-check

build:
	cargo build --all

release:
	cargo build --release

run:
	cargo run

test:
	bash pwn.sh
	cargo test

clean:
	cargo clean

fmt:
	cargo fmt

fmt-check:
	bash pwn.sh
	cargo fmt --all --check

clippy:
	cargo clippy --all --all-features -- -D warnings

taplo:
	taplo format

taplo-check:
	bash pwn.sh
	taplo format --check

deny-check:
	cargo deny --all-features check