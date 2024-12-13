autofix:
	npm run format

build:
	VITE_API_ROOT=https://yerevan.treemaps.app/ npm run build
	cp build/index.html build/404.html

clean:
	rm -rf node_modules

install:
	npm ci

lint:
	npm run lint

preview:
	npm run preview

serve: serve-dev
	VITE_API_ROOT=$(API_ROOT) npm run dev -- --host

serve-dev:
	VITE_API_ROOT=http://127.0.0.1:8000/ npm run dev -- --host

serve-live:
	VITE_API_ROOT=https://yerevan.treemaps.app/ npm run dev -- --host

serve-lan:
	VITE_API_ROOT=http://192.168.1.31:8000/ npm run dev -- --host

test:
	npm run test:unit -- run

.PHONY: build
