# frontend/Makefile

# =======================================
# TailYew Docs Frontend Makefile
# Located at: frontend/Makefile
# Purpose: Build and serve the static Yew + Tailwind documentation site
# =======================================

# ---------------------------------------
# Configuration
# ---------------------------------------
PORT ?= 3031                     # Default port for the development server

# =======================================
# Run and Serve Targets
# =======================================

# Start the Trunk dev server with hot reload
run:
	@echo "Starting Trunk dev server on http://localhost:$(PORT)..."
	trunk serve --config Trunk.toml --port $(PORT) \
		--watch src --watch static --watch ../crates/tailyew/src

# =======================================
# Cleanup Targets
# =======================================

clean:
	@echo "Cleaning up build artifacts..."
	rm -rf ./dist ./static/styles.css
	@echo "Cleanup completed."
