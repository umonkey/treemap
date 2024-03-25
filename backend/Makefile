DATABASE=var/database.sqlite

help:
	@echo "build - build the project"
	@echo "lint  - run clippy"
	@echo "serve - run the project"

build:
	cargo build --release

clean:
	rm -f *.profraw
	cargo clean

lint:
	cargo clippy

serve:
	cargo run

sqlite:
	sqlite3 $(DATABASE)

sqlite-schema:
	sqlite3 $(DATABASE) < dev/schema-sqlite.sql

sqlite-seed:
	sqlite3 $(DATABASE) < dev/seed-sqlite.sql

test:
ifneq (,$(FILTER))
	RUST_LOG="debug" cargo test -- $(FILTER) --nocapture --test-threads=1
else
	RUST_LOG="warn" cargo test
endif

# Some notes on code coverage:
# https://doc.rust-lang.org/rustc/instrument-coverage.html
# https://blog.rng0.io/how-to-do-code-coverage-in-rust/
test-coverage:
	RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="test.profraw" RUST_LOG="warn" cargo test
	grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
