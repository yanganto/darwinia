#!/usr/bin/env bash

set -eux

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=$RUST_TOOLCHAIN -y

source ~/.cargo/env

rustup --version
cargo --version
rustc --version

case $TARGET in
	# Format check in stable rust
	"rustfmt")
		cargo fmt --all
		;;

	# Without WASM, build then test
	"native")
		SKIP_WASM_BUILD=1 cargo build --release --all --locked "$@"
		;;

	# With WASM, build then test
	"wasm")
		rustup target add wasm32-unknown-unknown
		WASM_BUILD_TYPE=release cargo build --locked "$@"
		echo -e "\e[0;32m +------------+ \n | Build Pass | \n +------------+ \e[0m"
		;;
esac
