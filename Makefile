RM=rm-rs -rf


all: rebuild_bins rebuild_lib

rebuild_bins: clean build_bins_release

rebuild_lib: clean build_lib_release


build_bins:
	cargo build --bins --target-dir output/bins

build_bins_release:
	cargo build --release --bins --target-dir output/bins

build_lib:
	cargo build --lib --target-dir output/lib

build_lib_release:
	cargo build --release --lib --target-dir output/lib

install:
	cargo install rm-rs
	cargo install cargo-tarpaulin

update:
	cargo update

clean:
	cargo clean
	$(RM) output

fmt:
	cargo fmt --all

run:
	cargo run --bin rust_develop_template1
	cargo run --release --bin rust_develop_template2

lint:
	cargo clippy --all
	cargo clippy --all --no-default-features
	cargo clippy --all --tests
	cargo check --all --tests
	cargo check --all --benches

test:
	cargo test --all

bench:
	cargo bench --all

coverage:
	cargo tarpaulin --timeout=900 --branch --release
