help:
	@echo "make build         -- build the project"
	@echo "make clean         -- delete temporary files"
	@echo "make lint          -- run clippy"
	@echo "make serve         -- run the project"
	@echo "make test          -- run unit tests"

autofix:
	cargo clippy --fix

build:
	cargo build --release

check: lint
	cargo check
	cargo test -- --color=always

clean:
	cargo clean

lint:
	cargo clippy -- -D warnings
	cargo fmt --check

format:
	cargo fmt

serve:
	cargo run
