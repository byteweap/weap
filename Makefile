.PHONY: test build release clean

test:
	cargo test

build:
	cargo build --release

release:
	cargo publish --registry crates-io

clean:
	cargo clean
