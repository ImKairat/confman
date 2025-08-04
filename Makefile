
.PHONY: check
check:
	@cargo check

.PHONY: run
run:
	@cargo run --quiet

.PHONY: build
build:
	@cargo build --release

.PHONY: test
test:
	@cargo test --release

.PHONY: clean
clean:
	@rm -rf target