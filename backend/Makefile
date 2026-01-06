DATABASE=var/database.sqlite

help:
	@echo "make build         -- build the project"
	@echo "make clean         -- delete temporary files"
	@echo "make lint          -- run clippy"
	@echo "make serve         -- run the project"
	@echo "make sqlite        -- run the SQLite console"
	@echo "make sqlite-schema -- install the schema in SQLite"
	@echo "make sqlite-seed   -- install the test data"
	@echo "make test          -- run unit tests"
	@echo "make test-coverage -- run unit tests with coverage collection"

autofix:
	cargo clippy --fix

build:
	cargo build --release

clean:
	rm -f *.profraw
	cargo clean

lint:
	cargo clippy -- -D warnings
	cargo fmt --check

format:
	cargo fmt

queue:
	cargo run -- queue-consumer

serve:
	cargo run -- serve

sqlite:
	sqlite3 $(DATABASE)

sqlite-schema:
	sqlite3 $(DATABASE) < dev/schema-sqlite.sql

sqlite-seed:
	sqlite3 $(DATABASE) < dev/seed-sqlite.sql

test:
ifneq (,$(FILTER))
	RUST_LOG="debug" cargo test -- $(FILTER) --nocapture --test-threads=1 --color=always
else
	RUST_LOG="warn" cargo test -- --color=always
endif
	rm -f var/test-files/*

# Opens a shell to a running dev backend.
shell:
	docker exec -it trees-backend bash

# Some notes on code coverage:
# https://doc.rust-lang.org/rustc/instrument-coverage.html
# https://blog.rng0.io/how-to-do-code-coverage-in-rust/
test-coverage:
	RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="test.profraw" RUST_LOG="warn" cargo test
	grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
