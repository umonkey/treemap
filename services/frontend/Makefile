autofix:
	npm run autofix

build:
	npm run build
	cp build/index.html build/404.html

build-twa:
	npx @bubblewrap/cli update
	npx @bubblewrap/cli build --skipPwaValidation

clean:
	rm -rf node_modules .svelte-kit

deploy:
	npm run build
	npx wrangler pages deploy ./build --project-name=trees-of-yerevan --branch=master

format:
	npm run format

check:
	npm run check

icons:
	convert -resize 192x192 -strip -colorspace sRGB static/icons/tree-512x512.png static/icons/tree-192x192.png
	convert -resize 180x180 -strip -colorspace sRGB static/icons/tree-512x512.png static/icons/tree-180x180.png
	convert -resize 32x32 -strip -colorspace sRGB static/icons/tree-512x512.png static/icons/tree-32x32.png
	convert -resize 16x16 -strip -colorspace sRGB static/icons/tree-512x512.png static/icons/tree-16x16.png
	convert -resize 192x192 -strip -colorspace sRGB static/icons/app-512x512.png static/icons/app-192x192.png
	convert -resize 180x180 -strip -colorspace sRGB static/icons/app-512x512.png static/icons/app-180x180.png
	file static/icons/*.png

install:
	npm ci

lint:
	npm run lint
	npm run format-check

preview:
	npm run preview

preview-dev:
	npm run build -- --mode development
	npm run preview

serve:
	PUBLIC_API_ROOT="https://api.treemaps.app/" PUBLIC_FILE_BASE_URL="https://api.treemaps.app/v1/files/" npm run dev -- --host

storybook:
	npm run storybook

test:
ifneq (,$(FILTER))
	npm run test:unit -- run $(FILTER)
else
	npm run test:unit -- run
endif

screenshots:
	node scripts/generate-screenshots.js

auth:
	node scripts/generate-screenshots.js --auth

screenshots-setup:
	npm install
	npx playwright install chromium

.PHONY: build
