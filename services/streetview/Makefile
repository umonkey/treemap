SERVICE_NAME=treemap-pano

build:
	docker build -t $(SERVICE_NAME) .

shell:
	docker run --rm -it -v $(PWD):/var/app $(SERVICE_NAME):latest bash

lint:
	uv run flake8 .

typecheck:
	uv run mypy .

format:
	uv run isort .
	uv run black .
