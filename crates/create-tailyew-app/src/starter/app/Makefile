# =======================================
# Root Makefile: Orchestrator
# =======================================


# =======================================
# Utility Commands
# =======================================

format:
	@echo "Formatting all code with cargo fmt..."
	cargo fmt --all

lint:
	@echo "Linting all code with cargo clippy..."
	cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged -- -D warnings

pretty: format lint

# =======================================
# Frontend
# =======================================
FRONTEND_MAKE = $(MAKE) -C frontend

fe-%:
	@echo "Delegating to frontend/$*..."
	$(FRONTEND_MAKE) $*

run-frontend:
	make fe-run

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
	@echo "Frontend:"
	@echo "  make run-frontend     Run frontend dev server (Trunk hot reload)"
	@echo "  make fe-copy-tailyew-safelist   Refresh TailYew safelist HTML from local Cargo registry"
