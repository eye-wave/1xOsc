#!make
.PHONY: dev build post-build format lint test

dev:
	cargo watch -x build -s "make post-build"

build:
	cargo xtask bundle one_x_osc --release

post-build:
	cp target/bundled/*.clap ~/.clap
	cp target/bundled/*.vst3 ~/.vst3 -r

format:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test
