format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test

build:
	cargo build --release

run: 
	cargo run

all: format lint test build
