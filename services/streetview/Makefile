SERVICE_NAME=treemap-pano

build:
	docker build -t $(SERVICE_NAME) .

shell:
	mkdir -p output output-sfm
	docker run --rm -it -v $(PWD)/output:/dataset/images -v $(PWD)/output-sfm:/dataset/output -w /dataset $(SERVICE_NAME):latest bash

lint:
	uv run flake8 .

typecheck:
	uv run mypy .

format:
	uv run isort .
	uv run black .
