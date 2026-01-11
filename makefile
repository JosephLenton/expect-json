.PHONY: fmt lint test build publish docs codecov

fmt:
	cargo +stable fmt

fmt-check:
	cargo +stable fmt --check

lint:
	cargo +stable clippy

test:
	cargo +stable test

build:
	cargo +stable build

cargo-update:
	cargo +stable update

publish: cargo-update fmt-check lint test
	cargo +stable publish --package expect-json-macros
	cargo +stable update
	cargo +stable publish --package expect-json

docs:
	cargo +stable doc --open

codecov:
	cargo llvm-cov --open
