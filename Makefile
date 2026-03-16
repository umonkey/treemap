help:
	@echo "Development steps:"
	@echo ""
	@echo "make build                 -- compile backend and frontend"
	@echo "make start                 -- run the development version"

build:
	docker compose build

build-combined:
	docker build -t trees -f Dockerfile .

start:
	docker compose up

start-prod:
	docker compose -f compose.prod.yaml up
