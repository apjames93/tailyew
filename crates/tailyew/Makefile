# crates/tailyew/Makefile

# =======================================
# TailYew Component Library Makefile
# Located at: crates/tailyew/Makefile
# Purpose: Build, and document the reusable Yew + Tailwind component system
# =======================================

# ---------------------------------------
# Build the library
# ---------------------------------------
build:
	@echo "Building the tailyew crate..."
	cargo build

# ---------------------------------------
# Check for errors and warnings
# ---------------------------------------
check:
	@echo "Checking the tailyew crate for issues..."
	cargo check

# ---------------------------------------
# Build and open documentation
# ---------------------------------------
doc:
	@echo "Generating and opening documentation for tailyew..."
	cargo doc --no-deps --open

# Build docs without opening (useful for CI)
doc-no-open:
	@echo "Generating documentation for tailyew (no open)..."
	cargo doc --no-deps

# ---------------------------------------
# Clean build artifacts
# ---------------------------------------
clean:
	@echo "Cleaning tailyew build artifacts..."
	cargo clean

