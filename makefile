.PHONY: fmt lint test build publish codecov

fmt:
	cargo fmt

lint:
	cargo +stable clippy

test:
	cargo +stable test

build:
	cargo +stable build

publish: fmt lint test
	cargo publish --package expect-json-macros
	cargo publish --package expect-json

codecov:
	cargo llvm-cov --open
