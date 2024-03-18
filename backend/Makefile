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

test:
	cargo test
