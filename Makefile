SHELL := /bin/bash

.PHONY: run
run:
	@cargo run --release

.PHONY: build
build:
	@cargo build --release

.PHONY: test
test:
	@cargo test --release

.PHONY: clean
clean:
	@rm -rf target