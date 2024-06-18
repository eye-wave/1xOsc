#!make
.PHONY: preview dev build post-build format lint test

project_name := "one_x_osc"

preview:
	cargo watch -x run

dev:
	cargo watch -x "xtask bundle $(project_name)" -s "make post-build"

build:
	cargo xtask bundle $(project_name) --release
	$(MAKE) post-build

post-build:
	cp target/bundled/*.clap ~/.clap
	cp target/bundled/*.vst3 ~/.vst3 -r

format:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test
