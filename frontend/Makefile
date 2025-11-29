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
TAILWIND_CMD = npm run tailwind:build

# =======================================
# Build Targets
# =======================================

build-tailwind:
	@echo "Building Tailwind CSS..."
	$(TAILWIND_CMD)

build: clean
	@echo "Starting Trunk build..."
	make build-tailwind
	trunk build --config Trunk.toml --dist dist

# =======================================
# Run and Serve Targets
# =======================================

# Start the Trunk dev server with hot reload
run:
	@echo "Starting Trunk dev server on http://localhost:$(PORT)..."
	trunk serve --config Trunk.toml --port $(PORT) \
		--watch src --watch static --watch ../crates/tailyew/src
		--ignore static/styles.css --ignore static/pkg

# =======================================
# Cleanup Targets
# =======================================

clean:
	@echo "Cleaning up build artifacts..."
	rm -rf ./dist ./static/styles.css
	@echo "Cleanup completed."
