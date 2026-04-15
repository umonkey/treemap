CR_USER = umonkey

help:
	@echo "Development steps:"
	@echo ""
	@echo "make build                 -- compile backend and frontend"
	@echo "make start                 -- run the development version"

build:
	docker compose build

build-combined:
	docker build -t treemap:latest -f Dockerfile .

format-docs:
	npx -y prettier --write $(shell find ./docs ./.agents -name "*.md")

push:
	echo $(CR_TOKEN) | docker login ghcr.io -u $(CR_USER) --password-stdin
	docker tag treemap:latest ghcr.io/$(CR_USER)/treemap:latest
	docker push ghcr.io/$(CR_USER)/treemap:latest
	docker rmi ghcr.io/$(CR_USER)/treemap:latest

start:
	docker compose up

start-prod:
	docker compose -f compose.prod.yaml up
