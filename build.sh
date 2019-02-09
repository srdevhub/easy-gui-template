#!/bin/bash
set -eu

# Pre-requisites:
rustup target add wasm32-unknown-unknown
if ! [[ $(wasm-bindgen --version) ]]; then
	cargo update -p wasm-bindgen
	cargo install -f wasm-bindgen-cli
	cargo update
fi

BUILD=debug
# BUILD=release

# Clear output from old stuff:
rm -rf docs/*.wasm

function build_rust
{
	echo "Build rust:"
	cargo build --target wasm32-unknown-unknown

	echo "Generate JS bindings for wasm:"
	FOLDER_NAME=${PWD##*/}
	TARGET_NAME="example.wasm"
	wasm-bindgen "target/wasm32-unknown-unknown/$BUILD/$TARGET_NAME" \
	  --out-dir docs --no-modules --no-typescript
	  # --no-modules-global hoboho
}

build_rust
