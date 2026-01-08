autofix:
	npm run autofix

build:
	npm run build
	cp build/index.html build/404.html

clean:
	rm -rf node_modules

deploy:
	npm run build
	npx wrangler pages deploy ./build --project-name=trees-of-yerevan --branch=master

install:
	npm ci

lint:
	npm run lint
	npm run format-check

preview:
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

.PHONY: build
