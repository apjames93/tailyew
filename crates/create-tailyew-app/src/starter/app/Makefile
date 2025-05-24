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
	cargo watch -p frontend -w frontend/src -s 'make fe-run'

