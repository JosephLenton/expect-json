.PHONY: fmt build test lint publish

fmt:
	cargo fmt

build:
	cargo +stable build

test:
	cargo +stable test

lint:
	cargo +stable clippy

publish: fmt clippy test
	cargo publish --package expect-json-macros
	cargo publish --package expect-json
