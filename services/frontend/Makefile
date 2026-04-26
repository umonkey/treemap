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
	npm run dev -- --host

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
