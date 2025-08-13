.PHONY: fmt lint test build publish docs codecov

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

docs:
	cargo doc --open

codecov:
	cargo llvm-cov --open
