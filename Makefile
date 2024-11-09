.PHONY: all web

all:
	cargo build

web:
	cd svelte ; npm run build
