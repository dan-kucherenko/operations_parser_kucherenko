# Makefile for Expression Parser project

.PHONY: run test format lint check all

# Run the parser with a specified command
run:
	cargo run -- $(ARGS)

# Run all tests
test:
	cargo test

# Format the code
format:
	cargo fmt

# Lint the code
lint:
	cargo clippy -- -D warnings

# Run all checks before committing (format, lint, and tests)
check: format lint test

# Run format, lint, and tests
all: format lint test
