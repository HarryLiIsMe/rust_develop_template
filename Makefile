RM=rm-rs -rf


all: rebuild_bins rebuild_lib

rebuild_bins: clean build_bins_release

rebuild_lib: clean build_lib_release


build_bins:
	cargo build --bins --target-dir output/bins

build_bins_release:
	cargo build --bins --target-dir output/bins --release

build_lib:
	cargo build --lib --target-dir output/lib

build_lib_release:
	cargo build --lib --target-dir output/lib --release

install:
	cargo install rm-rs
	# cargo install cargo-tarpaulin

update:
	cargo update

clean:
	cargo clean
	$(RM) output

fmt:
	cargo fmt

lint:
	cargo clippy
	cargo clippy --no-default-features
	cargo clippy --tests
	cargo check --tests
	cargo check --benches

test:
	cargo test --release

bench:
	cargo bench

coverage:
	cargo tarpaulin --timeout=900 --branch --release



