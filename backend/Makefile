REVISION=$(shell git rev-parse --short HEAD)

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

test:
	mkdir -p var/test-files
ifneq (,$(FILTER))
	RUST_LOG="debug" cargo test -- $(FILTER) --nocapture --test-threads=1 --color=always
else
	RUST_LOG="warn" cargo test -- --color=always
	rm -f var/test-*.db
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

deploy-fly:
	fly auth docker
	docker build -t registry.fly.io/treemap-blue-frog-3927:$(REVISION) .
	docker push registry.fly.io/treemap-blue-frog-3927:$(REVISION)
	fly deploy --image registry.fly.io/treemap-blue-frog-3927:$(REVISION)
	docker rmi registry.fly.io/treemap-blue-frog-3927:$(REVISION)
