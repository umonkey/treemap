autofix:
	npm run autofix

build:
	npm run build
	cp build/index.html build/404.html

clean:
	rm -rf node_modules

deploy:
	$(MAKE) build
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
