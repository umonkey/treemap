API_ROOT ?= http://127.0.0.1:8000/

autofix:
	npm run autofix

build:
	VITE_API_ROOT=$(API_ROOT) VITE_ENVIRONMENT=production npm run build
	cp build/index.html build/404.html

clean:
	rm -rf node_modules

deploy:
	API_ROOT=https://yerevan.treemaps.app/ $(MAKE) build
	rm -rf ../../umonkey.github.io/*
	cp -r build/* ../../umonkey.github.io/
	git -C ../../umonkey.github.io/ add .
	git -C ../../umonkey.github.io/ commit -m "Deploy"
	git -C ../../umonkey.github.io/ push

install:
	npm ci

lint:
	npm run lint
	npm run format-check

preview:
	npm run preview

serve:
	VITE_API_ROOT=$(API_ROOT) VITE_ENVIRONMENT=development npm run dev -- --host

storybook:
	VITE_API_ROOT=$(API_ROOT) VITE_ENVIRONMENT=development npm run storybook

test:
ifneq (,$(FILTER))
	VITE_API_ROOT="https://api.treemaps.app/" npm run test:unit -- run $(FILTER)
else
	VITE_API_ROOT="https://api.treemaps.app/" npm run test:unit -- run
endif

.PHONY: build
