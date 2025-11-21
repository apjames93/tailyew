# =======================================
# Configuration
# =======================================
PORT ?= 9001                     # Default port for the development server
# TAILWIND_CMD = npm run tailwind:build

# =======================================
# Build Targets
# =======================================
copy-tailyew:
	@echo "🔁 Checking for local TailYew crate in Cargo registry..."
	@if find ~/.cargo/registry/src/ -type d -path "*/tailyew-*/src" | grep -q .; then \
		echo "📦 Found local TailYew source, copying..."; \
		rm -rf ./static/tailyew-safelist.html; \
		cp ~/.cargo/registry/src/*/tailyew-*/tailyew-safelist.html ./static/; \
	else \
		echo "⚠️ No TailYew source found in Cargo registry. Using existing ./static/tailyew..."; \
	fi

# =======================================
# Run and Serve Targets
# =======================================

# Start the Trunk dev server with hot reload
run:
	@echo "Starting Trunk dev server on http://localhost:$(PORT)..."
	trunk serve --config Trunk.toml --port $(PORT) \
		--watch src --watch static \
		--ignore static/styles.css --ignore static/pkg

# =======================================
# Cleanup Targets
# =======================================

# Clean up build artifacts
clean:
	@echo "Cleaning up build artifacts..."
	rm -rf ./dist ./static/styles.css
	@echo "Cleanup completed."
