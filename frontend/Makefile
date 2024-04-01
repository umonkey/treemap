help:
	@echo "make build         -- build the static files"
	@echo "make install       -- install the dependencies"
	@echo "make lint          -- run the linter"
	@echo "make serve         -- run the development server"
	@echo "make storybook     -- run the storybook server"
	@echo "make test          -- run the unit tests"
	@echo "make test-verbose  -- run the unit tests with console.log output"

build:
	npm run build

install:
	npm install

lint:
	npm run lint

serve:
	REACT_APP_API_ROOT=http://localhost:8000 npm run dev

storybook:
	npm run storybook -- --no-open

test:
	npm run test

test-verbose:
	npm run test-verbose
