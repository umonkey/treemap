help:
	@echo "build - build the project"
	@echo "lint  - run clippy"
	@echo "serve - run the project"

build:
	cargo build

lint:
	cargo clippy

serve:
	cargo run

sqlite:
	sqlite3 database.sqlite

sqlite-schema:
	sqlite3 database.sqlite < dev/schema-sqlite.sql

sqlite-seed:
	sqlite3 database.sqlite < dev/seed-sqlite.sql

# Some notes on code coverage:
# https://doc.rust-lang.org/rustc/instrument-coverage.html
# https://blog.rng0.io/how-to-do-code-coverage-in-rust/
test:
	RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="test.profraw" cargo test
	grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/

test-debug:
	cargo test -- --nocapture
