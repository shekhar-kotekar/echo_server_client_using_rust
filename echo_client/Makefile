.PHONY: build run

build:
	cargo fmt && cargo clippy && cargo check && cargo test && cargo build
run: build
	cargo run
