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

publish: fmt-check lint test
	cargo publish --package expect-json-macros
	cargo publish --package expect-json

docs:
	cargo doc --open

codecov:
	cargo llvm-cov --open
