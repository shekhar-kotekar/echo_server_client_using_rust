.PHONY: build run clean

clean:
	cargo clean
build:
	cargo fmt && cargo check && cargo test && cargo build
run: build
	cargo run
