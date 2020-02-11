#!/usr/bin/env bash

set -eux

source ~/.cargo/env

case $TARGET in
	# Format check in stable rust
	"rustfmt")
		echo -e "\e[0;32m +------------+ \n | No Test    | \n +------------+ \e[0m"
		;;

	# Without WASM, build then test
	"native")
		SKIP_WASM_BUILD=1 cargo test -p darwinia-kton "$@"
		echo -e "\e[0;32m +------------+ \n | Kton  Pass | \n +------------+ \e[0m"
		SKIP_WASM_BUILD=1 cargo test -p darwinia-ring "$@"
		echo -e "\e[0;32m +------------+ \n | Ring  Pass | \n +------------+ \e[0m"
		SKIP_WASM_BUILD=1 cargo test -p darwinia-staking "$@"
		echo -e "\e[0;32m +------------+ \n | Staking OK | \n +------------+ \e[0m"
		;;

	# With WASM, build then test
	"wasm")
		WASM_BUILD_TYPE=release cargo test -p darwinia-kton "$@"
		echo -e "\e[0;32m +------------+ \n | Kton  Pass | \n +------------+ \e[0m"
		WASM_BUILD_TYPE=release cargo test -p darwinia-ring "$@"
		echo -e "\e[0;32m +------------+ \n | Ring  Pass | \n +------------+ \e[0m"
		WASM_BUILD_TYPE=release cargo test -p darwinia-staking "$@"
		echo -e "\e[0;32m +------------+ \n | Staking OK | \n +------------+ \e[0m"
		;;
esac
