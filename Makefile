.PHONY: help run test build clean

help:
	@echo "Targets:"
	@echo "  make run ARGS=\"-- <input.md> [output.md]\""
	@echo "  make test"
	@echo "  make build"
	@echo "  make clean"

run:
	cargo run $(ARGS)

test:
	cargo test

build:
	cargo build

clean:
	cargo clean
