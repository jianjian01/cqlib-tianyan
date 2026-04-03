# cqlib-tianyan Workspace Makefile
#
# Usage:
#   make all       # Build all crates (Rust + Python wheel)
#   make rust      # Build only Rust crates (skips Python wheel)
#   make python    # Build Python wheel (requires maturin)
#   make c         # Build C bindings and run tests
#   make clean     # Clean all build artifacts
#   make test      # Run all tests (Rust + C)

.PHONY: all rust python c clean test rust-test c-test python-test

# Default target: build everything
all: rust python

# Build all Rust crates (excluding Python bindings which use maturin)
rust:
	cargo build --workspace --exclude binding-python
	cargo build --workspace --exclude binding-python --release

# Build Python wheel using maturin
python:
	cd crates/binding-python && maturin build --release

# Build and test C bindings
c:
	$(MAKE) -C crates/binding-c all
	$(MAKE) -C crates/binding-c test

# Run all tests
test: rust-test c-test

# Run Rust tests (excluding Python bindings)
rust-test:
	cargo test --workspace --exclude binding-python

# Run C binding tests
c-test:
	$(MAKE) -C crates/binding-c test

# Run Python tests (requires maturin develop first)
python-test: python
	cd crates/binding-python && maturin develop --release
	cd crates/binding-python && python -m pytest tests/ -v

# Clean all build artifacts
clean:
	cargo clean
	$(MAKE) -C crates/binding-c clean
	rm -rf crates/binding-python/target
	rm -rf crates/binding-python/dist

# Development build for Python (faster, no release optimizations)
python-dev:
	cd crates/binding-python && maturin develop

# Help
help:
	@echo "cqlib-tianyan Workspace Makefile"
	@echo ""
	@echo "Targets:"
	@echo "  all         - Build all crates (Rust + Python wheel)"
	@echo "  rust        - Build Rust crates only (excludes Python)"
	@echo "  python      - Build Python wheel (requires maturin)"
	@echo "  python-dev  - Development build for Python (faster)"
	@echo "  c           - Build and test C bindings"
	@echo "  test        - Run all tests (Rust + C)"
	@echo "  rust-test   - Run Rust tests only"
	@echo "  c-test      - Run C binding tests"
	@echo "  python-test - Run Python tests (builds + develops first)"
	@echo "  clean       - Clean all build artifacts"
