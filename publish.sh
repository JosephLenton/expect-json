#!/bin/bash

set -e

cargo fmt
cargo clippy
cargo test

cargo publish --package expect-json-macros
cargo publish --package expect-json
