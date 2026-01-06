help:
	@echo "Development steps:"
	@echo ""
	@echo "make build                 -- compile backend and frontend"
	@echo "make start                 -- run the development version"

build:
	docker compose build

start:
	docker compose up
