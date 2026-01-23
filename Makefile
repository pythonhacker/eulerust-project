# Makefile for Project Euler Rust workspace

# Directory where problem binaries live
BIN_DIR := euler-solutions/src/bin

# Extract and numerically sort problem names: problem1, problem2, ..., problem10, ...
PROBLEMS := $(shell ls $(BIN_DIR)/*.rs 2>/dev/null | \
	sed 's#.*/##' | sed 's/\.rs//' | \
	sort -t m -k2,2n)

.PHONY: run all list

# Run a single problem:
# make run problem11
run:
ifndef PROBLEM
	@echo "Usage: make run PROBLEM=problem11"
	@exit 1
endif
	@echo "Running $(PROBLEM)"
	cargo run -p euler-solutions --bin $(PROBLEM)

# Run all problems:
# make all
all:
	@for p in $(PROBLEMS); do \
		echo "=== Running $$p ==="; \
		cargo run -p euler-solutions --bin $$p || exit 1; \
	done

# Optional: list available problems
list:
	@echo "Available problems:"
	@for p in $(PROBLEMS); do echo "  $$p"; done
