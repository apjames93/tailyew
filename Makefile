# =======================================
# TailYew Root Makefile
# Purpose: Orchestrate frontend and component crate build/dev/test/docs
# =======================================

# ---------------------------------------
# Include Submodule Makefiles
# ---------------------------------------
include frontend/Makefile
include crates/tailyew/Makefile
include aws/Makefile


# ---------------------------------------
# Directory-specific Delegates
# ---------------------------------------
AWS_MAKE = $(MAKE) -C aws
FRONTEND_MAKE   = $(MAKE) -C frontend
TAIL_YEW_MAKE   = $(MAKE) -C crates/tailyew

# ---------------------------------------
# General Dev Utilities
# ---------------------------------------

# Format all Rust code
format:
	@echo "📐 Formatting code..."
	cargo fmt --all

# Run Clippy with all features and deny warnings
lint:
	@echo "🕵️  Linting code..."
	cargo clippy --workspace --lib --bins --tests --all-features -- -D warnings

# Format and lint together
pretty: format lint

# ---------------------------------------
# Component System (Tailyew) Commands
# ---------------------------------------

# Build and generate docs for tailyew
build-tailyew:
	make tailyew-build
	make tailyew-doc

# Watch and regenerate tailyew Rust API docs
watch-docs:
	@echo "📚 Watching for API doc changes..."
	cargo watch -p tailyew -w crates/tailyew/src -s 'make tailyew-doc'

release-check:
	cargo check -p tailyew
	make pretty
	cd crates/tailyew && cargo publish --dry-run

# Publish the tailyew crate to crates.io
publish-tailyew:
	@echo "🚀 Publishing tailyew to crates.io..."
	cargo check -p tailyew
	cd crates/tailyew && cargo publish --allow-dirty


# ---------------------------------------
# Create-Tailyew-App Commands
# ---------------------------------------

CTYA := crates/create-tailyew-app

.PHONY: release-check-create-tailyew-app publish-create-tailyew-app

release-check-create-tailyew-app:
	@echo "🔎 Dry‐running create-tailyew-app"
	cd $(CTYA) && cargo check
	cd $(CTYA) && cargo publish --dry-run

publish-create-tailyew-app:
	@echo "🚀 Publishing create-tailyew-app to crates.io…"
	cd $(CTYA) && cargo check
	cd $(CTYA) && cargo publish

# ---------------------------------------
# Frontend Docs Site Commands
# ---------------------------------------

# Watch frontend and tailyew together (docs site DX)
run-frontend:
	@echo "🚀 Running TailYew docs site with Trunk (hot reloading)..."
	cd frontend && trunk serve --config Trunk.toml --port 3031 \
		--watch src --watch static --watch ../crates/tailyew/src

# ---------------------------------------
# Command Delegation Helpers
# ---------------------------------------

# Delegate to any frontend target with `make fe-<target>`
fe-%:
	@echo "🔧 Delegating to frontend/$*..."
	$(FRONTEND_MAKE) $*

# Delegate to any tailyew target with `make tailyew-<target>`
tailyew-%:
	@echo "🔧 Delegating to crates/tailyew/$*..."
	$(TAIL_YEW_MAKE) $*

aws-%:
	@echo "🔧 Delegating to aws/$*..."
	$(AWS_MAKE) $*

# ---------------------------------------
# Help Message
# ---------------------------------------

help:
	@echo "🛠️  Available Make targets:"
	@echo ""
	@echo "General:"
	@echo "  make format           Format all code"
	@echo "  make lint             Lint with clippy (warnings = errors)"
	@echo "  make pretty           Format + lint"
	@echo ""
	@echo "Component Crate (tailyew):"
	@echo "  make build-tailyew    Build and generate docs for tailyew"
	@echo "  make watch-docs       Watch and auto-generate Rust docs"
	@echo "  make publish-tailyew  Publish the crate to crates.io"
	@echo ""
	@echo "Frontend Docs Site:"
	@echo "  make run-frontend     Watch frontend + tailyew (hot reload)"
	@echo "  make fe-<target>      Run a target from frontend/Makefile"
	@echo "  make tailyew-<target> Run a target from crates/tailyew/Makefile"
