#!make
.PHONY: preview dev build post-build format lint test

UNAME := $(shell uname)
PROJECT_NAME := "one_x_osc"

VST3_PATH := ~/.vst3
CLAP_PATH := ~/.clap

ifeq ($(OS),Windows_NT)
	VST3_PATH := "C:/Program Files/Common Files/VST3/"
	CLAP_PATH := "C:/Program Files/Common Files/CLAP/"
endif

ifeq ($(UNAME), Darwin)
	VST3_PATH := ~/Library/Audio/Plug-Ins/VST3
	CLAP_PATH := ~/Library/Audio/Plug-Ins/CLAP
endif

preview:
	cargo watch -x run

dev:
	cargo watch -x "xtask bundle $(PROJECT_NAME)" -s "make post-build"

build:
	cargo xtask bundle $(PROJECT_NAME) --release
	$(MAKE) post-build

post-build:
	@mkdir -p $(VST3_PATH)
	@mkdir -p $(CLAP_PATH)

	cp target/bundled/*.clap $(CLAP_PATH)
	cp target/bundled/*.vst3 $(VST3_PATH) -r

format:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test
