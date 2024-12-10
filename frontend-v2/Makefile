API_ROOT ?= http://127.0.0.1:8000/

autofix:
	npm run format

build:
	npm run build

clean:
	rm -rf node_modules

lint:
	npm run lint

serve:
	VITE_API_ROOT=$(API_ROOT) npm run dev -- --host

test:
	npm run test:unit -- run

.PHONY: build
